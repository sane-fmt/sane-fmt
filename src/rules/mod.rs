pub use dprint_plugin_typescript::configuration::{
    Configuration as Cfg, ConfigurationBuilder as CfgBuilder,
};

use dprint_plugin_typescript::{
    configuration::{QuoteStyle, SemiColonOrComma, SemiColons, TrailingCommas, UseParentheses},
    format_text,
};
use pipe_trait::*;
use std::path::Path;

/// Shape a `ConfigurationBuilder` to desired configuration.
pub fn modify(builder: &mut CfgBuilder) -> &mut CfgBuilder {
    builder
        .deno()
        .line_width(120)
        .quote_style(QuoteStyle::PreferSingle)
        .semi_colons(SemiColons::Asi)
        .type_literal_separator_kind_single_line(SemiColonOrComma::Comma)
        .trailing_commas(TrailingCommas::OnlyMultiLine)
        .arrow_function_use_parentheses(UseParentheses::PreferNone)
        .ignore_node_comment_text("sane-fmt-ignore")
        .ignore_file_comment_text("sane-fmt-ignore-file")
}

/// Create desired configuration.
pub fn build_cfg() -> Cfg {
    CfgBuilder::new().pipe_mut(modify).build()
}

/// Create a formatter for desired configuration.
pub fn build_fmt() -> Fmt {
    Fmt(build_cfg())
}

/// Wrapper type of configuration.
pub struct Fmt(Cfg);

impl Fmt {
    /// Format a file
    pub fn format_text(&self, path: &Path, content: &str) -> Result<String, String> {
        format_text(path, content, &self.0)
    }

    /// Extract internal configuration
    pub fn into_cfg(self) -> Cfg {
        self.0
    }
}
