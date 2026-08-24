mod adapters;
mod application;
mod error;
mod ports;
mod server;

pub(crate) use adapters::{ToolSurface, catalog_for_surface};
pub use error::McpError;
pub(crate) use server::serve_with_surface;
