use crate::mcp::McpError;
use crate::mcp::adapters::ToolSurface;
use crate::write_gate::WriteGate;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

#[cfg(test)]
use crate::mcp::adapters::{NotifyMonitorFactory, RefactorRepository, ToolCatalog};
#[cfg(test)]
use crate::mcp::application::RepositorySession;
#[cfg(test)]
use mcport::{ServerIdentity, ToolPayload, ToolReply, ToolServer, Value};

/// The selected Refactor tool surface behind the shared `mcport` stdio runtime.
///
/// Refactor exports only its own methods while using the Rust graph internally.
#[cfg(test)]
struct RefactorServer {
    identity: ServerIdentity,
    catalog: Value,
    tool_names: std::collections::BTreeSet<String>,
    session: RepositorySession,
}

#[cfg(test)]
impl RefactorServer {
    #[cfg(test)]
    fn new(root: impl AsRef<Path>, gate: WriteGate) -> Result<Self, McpError> {
        Self::new_for_surface(root, gate, ToolSurface::Refactor)
    }

    fn new_for_surface(
        root: impl AsRef<Path>,
        gate: WriteGate,
        surface: ToolSurface,
    ) -> Result<Self, McpError> {
        let catalog = ToolCatalog::for_surface(surface)?;
        let instructions = match surface {
            ToolSurface::Refactor => {
                "Refactor extension only: twelve evidence-backed planning, preview, apply, and \
                 rollback tools. Use the separate Weavatrix core server for read-only repository \
                 intelligence. Writes require an explicit gate and a plan-bound single-use token."
            }
            ToolSurface::Rename => {
                "Rename-only workflow. Start with the bare symbol name; if it is ambiguous, retry \
                 rename_symbol with one candidate id. Do not invent a symbol id. Preview the exact \
                 candidate, then repeat the identical symbol and new_name with mode=apply and its \
                 confirm_token. rollback_last_apply is the recovery tool."
            }
        };
        let repository = RefactorRepository::open(
            root.as_ref().to_path_buf(),
            catalog.refactor_names.clone(),
            gate,
        )?;
        Ok(Self {
            identity: ServerIdentity::new(
                "weavatrix-refactor",
                env!("CARGO_PKG_VERSION"),
                instructions,
            ),
            catalog: catalog.encoded,
            tool_names: catalog.names,
            session: RepositorySession::new(Box::new(repository), Arc::new(NotifyMonitorFactory)),
        })
    }
}

#[cfg(test)]
impl ToolServer for RefactorServer {
    fn identity(&self) -> ServerIdentity {
        self.identity.clone()
    }

    fn identity_ref(&self) -> Option<&ServerIdentity> {
        Some(&self.identity)
    }

    fn catalog(&mut self) -> Value {
        self.catalog.clone()
    }

    fn catalog_ref(&mut self) -> Option<&Value> {
        Some(&self.catalog)
    }

    fn has_tool(&self, name: &str) -> Option<bool> {
        Some(self.tool_names.contains(name))
    }

    fn call(&mut self, name: &str, arguments: Value) -> ToolReply {
        if !self.tool_names.contains(name) {
            return ToolReply::error(format!("tool `{name}` is not exposed by this MCP profile"));
        }
        // `Mirrored` rather than `Structured`: it carries structuredContent *and* the text
        // mirror, which is what the old boolean did. Dropping the mirror would show an empty
        // result to any client that reads only `content`.
        let payload = if arguments
            .get("output_format")
            .and_then(Value::as_str)
            .is_none_or(|format| format == "json")
        {
            ToolPayload::Mirrored
        } else {
            ToolPayload::Text
        };
        match self.session.call(name, arguments) {
            Ok(value) => ToolReply::Success { value, payload },
            Err(error) => ToolReply::error(error),
        }
    }
}

/// Serves the selected refactor or rename surface over stdio.
///
/// The launch root is checked before the handshake. Graphs stay lazy: the first call that names
/// a repository opens that lane, so one large tree does not block another agent's first call.
/// The write gate is read once at startup rather than per call.
///
/// # Errors
///
/// Returns stdio failures or a missing launch root.
pub(crate) fn serve_with_surface(
    root: impl AsRef<Path>,
    gate: WriteGate,
    surface: ToolSurface,
    allow_retarget: bool,
) -> Result<(), McpError> {
    let server = parallel::ParallelServer::open(root, gate, surface, allow_retarget)?;
    let config = mcport::RuntimeConfig {
        max_in_flight: IN_FLIGHT,
        queue_depth: QUEUE_DEPTH,
        output_queue_depth: QUEUE_DEPTH,
        handler_deadline: Some(Duration::from_secs(HANDLER_DEADLINE_SECS)),
        ..mcport::RuntimeConfig::default()
    };
    mcport::serve_controlled(Arc::new(server), config).map_err(McpError::Io)
}

mod parallel;
mod routing;

pub(crate) use parallel::{HANDLER_DEADLINE_SECS, IN_FLIGHT, QUEUE_DEPTH};

#[cfg(test)]
mod lanes;
#[cfg(test)]
mod tests;
