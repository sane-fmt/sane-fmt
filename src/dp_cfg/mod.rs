pub use dprint_plugin_typescript::{
    configuration::{Configuration as Cfg, ConfigurationBuilder as CfgBuilder},
    Formatter as Fmt,
};

/// Shape a `ConfigurationBuilder` to desired configuration
pub fn modify(builder: &mut CfgBuilder) -> &mut CfgBuilder {
    builder.deno()
}

/// Create desired configuration
pub fn build_cfg() -> Cfg {
    let mut builder = CfgBuilder::new();
    modify(&mut builder);
    builder.build()
}

/// Create a formatter for desired configuration
pub fn build_fmt() -> Fmt {
    Fmt::new(build_cfg())
}
