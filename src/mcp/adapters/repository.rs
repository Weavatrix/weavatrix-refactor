#[cfg(test)]
use crate::mcp::ports::RepositoryPort;
#[cfg(test)]
use crate::write_gate::WriteGate;
use blazingly_json::Value;
use std::collections::BTreeSet;
use std::io;
#[cfg(test)]
use std::path::{Path, PathBuf};
#[cfg(test)]
use weavatrix_rust::{Error, Weavatrix};
use weavatrix_rust_refactor::operations as refactor;

/// Catalog builder for the 12-tool extension or the 2-tool rename workflow.
///
/// Refactor uses the read-only engine internally but never exports its methods.
pub(crate) struct ToolCatalog {
    pub(crate) encoded: Value,
    pub(crate) names: BTreeSet<String>,
    /// Kept for the single-repository server the catalog tests still drive.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) refactor_names: BTreeSet<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ToolSurface {
    #[default]
    Refactor,
    Rename,
}

impl ToolSurface {
    pub(crate) fn from_profile(profile: &str) -> Option<Self> {
        match profile {
            "refactor" => Some(Self::Refactor),
            "rename" => Some(Self::Rename),
            _ => None,
        }
    }
}

pub(crate) fn catalog_for_surface(surface: ToolSurface) -> io::Result<Value> {
    ToolCatalog::for_surface(surface).map(|catalog| catalog.encoded)
}

impl ToolCatalog {
    pub(crate) fn for_surface(surface: ToolSurface) -> io::Result<Self> {
        let all_refactor_names = refactor::catalog_names()
            .into_iter()
            .collect::<BTreeSet<_>>();
        let refactor_names = match surface {
            ToolSurface::Refactor => all_refactor_names.clone(),
            ToolSurface::Rename => ["rename_symbol", "rollback_last_apply"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
        };
        let mut encoded = Vec::new();
        if let Some(tools) = refactor::catalog().as_array() {
            encoded.extend(
                tools
                    .iter()
                    .filter(|tool| {
                        tool.get("name")
                            .and_then(Value::as_str)
                            .is_some_and(|name| refactor_names.contains(name))
                    })
                    .cloned(),
            );
        }
        if !refactor_names.is_subset(&all_refactor_names) || encoded.len() != refactor_names.len() {
            return Err(io::Error::other(
                "requested tool surface is not present in the refactor contract",
            ));
        }
        Ok(Self {
            encoded: Value::Array(encoded),
            names: refactor_names.clone(),
            refactor_names,
        })
    }
}

#[cfg(test)]
pub(crate) struct RefactorRepository {
    root: PathBuf,
    engine: Option<Weavatrix>,
    refactor_names: BTreeSet<String>,
    /// Confirmations live for the life of the server, not the call: a token issued by a preview
    /// has to still be there when the apply arrives.
    session: refactor::RefactorSession,
}

#[cfg(test)]
impl RefactorRepository {
    pub(crate) fn open(
        root: PathBuf,
        refactor_names: BTreeSet<String>,
        gate: WriteGate,
    ) -> Result<Self, Error> {
        let engine = Weavatrix::open(&root)?;
        Ok(Self {
            root,
            engine: Some(engine),
            refactor_names,
            session: refactor::RefactorSession::new(gate.is_open()),
        })
    }

    fn engine(&mut self) -> Result<&mut Weavatrix, Error> {
        if self.engine.is_none() {
            let engine = Weavatrix::open(&self.root)?;
            engine.state().warm_communities();
            self.engine = Some(engine);
        }
        self.engine
            .as_mut()
            .ok_or_else(|| Error::InvalidRepository(self.root.clone()))
    }

    fn refresh(&mut self) -> Result<bool, String> {
        let engine = self
            .engine
            .as_mut()
            .ok_or_else(|| "repository graph is not initialized".to_owned())?;
        let refreshed = engine
            .refresh_if_stale()
            .map_err(|error| format!("repository refresh failed: {error}"))?;
        if refreshed {
            engine.state().warm_communities();
        }
        Ok(refreshed)
    }
}

#[cfg(test)]
impl RepositoryPort for RefactorRepository {
    fn root(&self) -> &Path {
        &self.root
    }

    fn is_loaded(&self) -> bool {
        self.engine.is_some()
    }

    fn ensure_loaded(&mut self) -> Result<(), String> {
        self.engine().map(|_| ()).map_err(|error| error.to_string())
    }

    fn refresh_if_stale(&mut self) -> Result<bool, String> {
        self.refresh()
    }

    fn call(&mut self, name: &str, arguments: Value) -> Result<Value, String> {
        if self.refactor_names.contains(name) {
            self.ensure_loaded()?;
            let state = self
                .engine
                .as_ref()
                .ok_or_else(|| "repository graph is not initialized".to_owned())?
                .state();
            return self.session.call(state, name, &arguments);
        }
        Err(format!(
            "tool `{name}` is not part of the Weavatrix Refactor extension"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{ToolCatalog, ToolSurface};
    use blazingly_json::Value;
    use weavatrix_rust::{Weavatrix, operations};
    use weavatrix_rust_refactor::operations as refactor;

    #[test]
    fn the_product_architecture_passes_its_internal_engine() {
        let mut engine = Weavatrix::open(env!("CARGO_MANIFEST_DIR")).expect("open repository");
        let result = operations::call(
            &mut engine,
            "verify_architecture",
            blazingly_json::json!({}),
        )
        .expect("verify architecture");
        assert_eq!(result.get("state").and_then(Value::as_str), Some("PASS"));
        for bucket in ["new", "existing", "excepted"] {
            assert_eq!(
                result.get(bucket).and_then(Value::as_array).map(Vec::len),
                Some(0),
                "{bucket} architecture violations"
            );
        }
    }

    #[test]
    fn the_default_catalog_contains_only_the_refactor_contract() {
        let catalog = ToolCatalog::for_surface(ToolSurface::Refactor).expect("refactor catalog");
        for name in refactor::catalog_names() {
            assert!(
                catalog.names.contains(&name),
                "{name} missing from the refactor catalog"
            );
            assert!(catalog.refactor_names.contains(&name));
        }
        assert!(!catalog.names.contains("graph_stats"));
        let encoded = catalog.encoded.as_array().expect("catalog is an array");
        assert_eq!(encoded.len(), catalog.names.len());
    }

    #[test]
    fn the_refactor_half_is_exactly_twelve_tools() {
        let catalog = ToolCatalog::for_surface(ToolSurface::Refactor).expect("refactor catalog");
        assert_eq!(catalog.refactor_names.len(), 12);
    }

    #[test]
    fn the_refactor_surface_exposes_only_the_twelve_extension_tools() {
        let catalog = ToolCatalog::for_surface(ToolSurface::Refactor).expect("refactor catalog");
        assert_eq!(catalog.names, catalog.refactor_names);
        assert_eq!(catalog.names.len(), 12);
        assert!(!catalog.names.contains("graph_stats"));
        assert_eq!(catalog.encoded.as_array().map(Vec::len), Some(12));
    }

    #[test]
    fn the_default_surface_is_the_refactor_extension() {
        assert_eq!(ToolSurface::default(), ToolSurface::Refactor);
        assert_eq!(ToolSurface::from_profile("full"), None);
        assert_eq!(
            ToolSurface::from_profile("refactor"),
            Some(ToolSurface::Refactor)
        );
        assert_eq!(
            ToolSurface::from_profile("rename"),
            Some(ToolSurface::Rename)
        );
        assert_eq!(ToolSurface::from_profile("unknown"), None);
    }

    #[test]
    fn the_rename_surface_exposes_only_the_safe_workflow_and_recovery() {
        let catalog = ToolCatalog::for_surface(ToolSurface::Rename).expect("rename catalog");
        assert_eq!(
            catalog.names,
            ["rename_symbol".to_owned(), "rollback_last_apply".to_owned()]
                .into_iter()
                .collect()
        );
        assert_eq!(catalog.refactor_names, catalog.names);
        assert_eq!(catalog.encoded.as_array().map(Vec::len), Some(2));
    }
}
