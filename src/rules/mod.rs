use dprint_plugin_typescript::configuration::{
    QuoteStyle, SemiColons, TrailingCommas, UseParentheses,
};
pub use dprint_plugin_typescript::{
    configuration::{Configuration as Cfg, ConfigurationBuilder as CfgBuilder},
    Formatter as Fmt,
};

/// Shape a `ConfigurationBuilder` to desired configuration.
pub fn modify(builder: &mut CfgBuilder) -> &mut CfgBuilder {
    builder
        .deno()
        .quote_style(QuoteStyle::PreferSingle)
        .semi_colons(SemiColons::Asi)
        .trailing_commas(TrailingCommas::OnlyMultiLine)
        .arrow_function_use_parentheses(UseParentheses::PreferNone)
        .union_and_intersection_type_prefer_hanging(true)
}

/// Create desired configuration.
pub fn build_cfg() -> Cfg {
    let mut builder = CfgBuilder::new();
    modify(&mut builder);
    builder.build()
}

/// Create a formatter for desired configuration.
pub fn build_fmt() -> Fmt {
    Fmt::new(build_cfg())
}
