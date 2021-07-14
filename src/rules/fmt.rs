use dprint_plugin_typescript::{configuration::Configuration as Cfg, format_text};
use std::path::Path;

/// Wrapper type of configuration.
pub struct Fmt(Cfg);

impl Fmt {
    /// Create a formatter
    pub fn from_cfg(cfg: Cfg) -> Self {
        Fmt(cfg)
    }

    /// Extract internal configuration
    pub fn into_cfg(self) -> Cfg {
        self.0
    }

    /// Format a file
    pub fn format_text(&self, path: &Path, content: &str) -> Result<String, String> {
        format_text(path, content, &self.0).map_err(|error| error.to_string())
    }
}
