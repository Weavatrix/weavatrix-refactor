//! One lane per repository. Agents share this process and do not share tokens.

use super::routing;
use crate::mcp::McpError;
use crate::mcp::adapters::{ToolCatalog, ToolSurface};
use crate::write_gate::WriteGate;
use blazingly_json::Value;
use mcport::{ConcurrentToolServer, RequestContext, ServerIdentity, ToolPayload, ToolReply};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use weavatrix_rust::Weavatrix;
use weavatrix_rust_refactor::operations as refactor;

/// Handler threads for one stdio process.
pub const IN_FLIGHT: usize = 128;
/// Calls waiting for a handler.
pub const QUEUE_DEPTH: usize = 256;
/// Cold graphs on a large tree can exceed the mcport default of 30 seconds.
pub const HANDLER_DEADLINE_SECS: u64 = 120;

const LANE_IDLE: Duration = Duration::from_secs(20 * 60);

struct Lane {
    inner: Mutex<LaneInner>,
}

struct LaneInner {
    engine: Option<Weavatrix>,
    session: refactor::RefactorSession,
    used: Instant,
}

/// Concurrent refactor surface. Each repository has its own graph, token store, and rollback.
pub struct ParallelServer {
    identity: ServerIdentity,
    catalog: Value,
    tool_names: std::collections::BTreeSet<String>,
    launch: PathBuf,
    allow_retarget: bool,
    write_allowed: bool,
    lanes: Mutex<BTreeMap<PathBuf, Arc<Lane>>>,
}

impl ParallelServer {
    /// Pins `root` for the process lifetime and leaves every graph unbuilt.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when `root` is missing or not a directory, or when the selected
    /// tool surface is not in the refactor contract.
    pub fn open(
        root: impl AsRef<Path>,
        gate: WriteGate,
        surface: ToolSurface,
        allow_retarget: bool,
    ) -> Result<Self, McpError> {
        let root = root.as_ref();
        if !root.is_dir() {
            return Err(McpError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("repository root {} is not a directory", root.display()),
            )));
        }
        let launch = root.canonicalize().map_err(McpError::Io)?;
        let catalog = ToolCatalog::for_surface(surface).map_err(McpError::Io)?;
        let instructions = match surface {
            ToolSurface::Refactor => {
                "Refactor extension only: twelve evidence-backed planning, preview, apply, and \
                 rollback tools. Use the separate Weavatrix core server for read-only repository \
                 intelligence. Writes require an explicit gate and a plan-bound single-use token. \
                 Pass repository to select a child of the launch root. Each repository has its own \
                 graph, confirm token, and rollback."
            }
            ToolSurface::Rename => {
                "Rename-only workflow. Start with the bare symbol name; if it is ambiguous, retry \
                 rename_symbol with one candidate id. Do not invent a symbol id. Preview the exact \
                 candidate, then repeat the identical symbol and new_name with mode=apply and its \
                 confirm_token. rollback_last_apply is the recovery tool. Pass repository to select \
                 a child of the launch root; a token from one repository does not apply to another."
            }
        };
        eprintln!(
            "weavatrix-refactor: pinned repository {} (retarget {}); {IN_FLIGHT} calls in flight",
            launch.display(),
            if allow_retarget { "allowed" } else { "blocked" }
        );
        Ok(Self {
            identity: ServerIdentity::new(
                "weavatrix-refactor",
                env!("CARGO_PKG_VERSION"),
                instructions,
            ),
            catalog: catalog.encoded,
            tool_names: catalog.names,
            launch,
            allow_retarget,
            write_allowed: gate.is_open(),
            lanes: Mutex::new(BTreeMap::new()),
        })
    }

    /// Answers one tool call. `repository` selects the lane; the launch pin stays.
    pub fn answer(&self, name: &str, mut arguments: Value) -> ToolReply {
        if !self.tool_names.contains(name) {
            return ToolReply::error(format!("tool `{name}` is not exposed by this MCP profile"));
        }
        let payload = if arguments
            .get("output_format")
            .and_then(Value::as_str)
            .is_none_or(|format| format == "json")
        {
            ToolPayload::Mirrored
        } else {
            ToolPayload::Text
        };
        let target = match self.resolve(&arguments) {
            Ok(target) => target,
            Err(error) => return ToolReply::error(error),
        };
        if let Err(error) = routing::accept_expected(&arguments, &self.launch, &target) {
            return ToolReply::error(error);
        }
        match self.on_lane(&target, name, &mut arguments) {
            Ok(value) => ToolReply::Success { value, payload },
            Err(error) => ToolReply::error(error),
        }
    }

    fn on_lane(&self, target: &Path, name: &str, arguments: &mut Value) -> Result<Value, String> {
        let lane = self.lane_for(target);
        let mut inner = lock(&lane.inner);
        inner.used = Instant::now();
        // After the lane mutex, so two calls in this process wait their turn. Before the graph
        // refresh, so a reader cannot scan a file another process is writing. The session takes
        // the same lock again on this thread.
        let _guard = match refactor::repository_guard(target, name, arguments, self.write_allowed) {
            Ok(guard) => guard,
            Err(refusal) => return Ok(stamp_repository(refusal, target)),
        };
        ensure_engine(&mut inner, target)?;
        routing::strip_routing(arguments);
        let state = inner
            .engine
            .as_ref()
            .ok_or_else(|| format!("repository graph is not initialized: {}", target.display()))?
            .state();
        let value = inner.session.call(state, name, arguments)?;
        Ok(stamp_repository(value, target))
    }

    fn lane_for(&self, target: &Path) -> Arc<Lane> {
        let mut lanes = lock(&self.lanes);
        let now = Instant::now();
        lanes.retain(|path, lane| path == &self.launch || path == target || !lane_idle(lane, now));
        lanes
            .entry(target.to_path_buf())
            .or_insert_with(|| {
                Arc::new(Lane {
                    inner: Mutex::new(LaneInner {
                        engine: None,
                        session: refactor::RefactorSession::new(self.write_allowed),
                        used: Instant::now(),
                    }),
                })
            })
            .clone()
    }

    fn resolve(&self, arguments: &Value) -> Result<PathBuf, String> {
        let Some(requested) = routing::text(arguments, "repository") else {
            return Ok(self.launch.clone());
        };
        let raw = Path::new(requested);
        let joined = if raw.is_absolute() {
            raw.to_path_buf()
        } else {
            self.launch.join(raw)
        };
        let canonical = joined.canonicalize().map_err(|_| {
            format!(
                "repository {requested} is not a directory inside {}",
                self.launch.display()
            )
        })?;
        if !canonical.is_dir() {
            return Err(format!("repository {requested} is not a directory"));
        }
        if canonical == self.launch || canonical.starts_with(&self.launch) {
            return Ok(canonical);
        }
        if self.allow_retarget {
            return Ok(canonical);
        }
        Err(format!(
            "repository {requested} is outside the launch root {}. Pass a child directory, or \
             relaunch with --allow-retarget",
            self.launch.display()
        ))
    }
}

impl ConcurrentToolServer for ParallelServer {
    fn identity(&self) -> ServerIdentity {
        self.identity.clone()
    }

    fn identity_ref(&self) -> Option<&ServerIdentity> {
        Some(&self.identity)
    }

    fn catalog(&self) -> Value {
        self.catalog.clone()
    }

    fn catalog_ref(&self) -> Option<&Value> {
        Some(&self.catalog)
    }

    fn has_tool(&self, name: &str) -> Option<bool> {
        Some(self.tool_names.contains(name))
    }

    fn call(&self, _context: &RequestContext, name: &str, arguments: Value) -> ToolReply {
        self.answer(name, arguments)
    }
}

fn stamp_repository(mut value: Value, target: &Path) -> Value {
    if let Some(object) = value.as_object_mut() {
        object.insert(
            "repository".to_owned(),
            blazingly_json::json!(target.display().to_string()),
        );
    }
    value
}

fn ensure_engine(inner: &mut LaneInner, target: &Path) -> Result<(), String> {
    if inner.engine.is_none() {
        let engine = Weavatrix::open(target).map_err(|error| error.to_string())?;
        engine.state().warm_communities();
        inner.engine = Some(engine);
        return Ok(());
    }
    let engine = inner
        .engine
        .as_mut()
        .ok_or_else(|| format!("repository graph is not initialized: {}", target.display()))?;
    let refreshed = engine
        .refresh_if_stale()
        .map_err(|error| format!("repository refresh failed: {error}"))?;
    if refreshed {
        engine.state().warm_communities();
    }
    Ok(())
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A lane another call is using stays. `try_lock` avoids waiting on that call while the map
/// lock is held, which would stall every other repository.
fn lane_idle(lane: &Lane, now: Instant) -> bool {
    let Ok(inner) = lane.inner.try_lock() else {
        return false;
    };
    now.saturating_duration_since(inner.used) >= LANE_IDLE
}
