mod fmt;

pub use dprint_plugin_typescript::configuration::{
    Configuration as Cfg, ConfigurationBuilder as CfgBuilder,
};
pub use fmt::Fmt;

use dprint_plugin_typescript::configuration::{
    QuoteStyle, SemiColonOrComma, SemiColons, TrailingCommas, UseParentheses,
};
use pipe_trait::*;

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
    Fmt::new(build_cfg())
}
