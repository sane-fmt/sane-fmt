pub mod schema_url;
pub mod typescript;

pub use dprint_plugin_typescript::configuration::Configuration as TypeScriptCfg;
pub use typescript::TypeScriptCfgWithSchema;

use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DprintCfg {
    #[serde(rename = "$schema")]
    schema_url: schema_url::DprintCoreSchemaUrl,

    typescript: TypeScriptCfg,
}

impl From<TypeScriptCfg> for DprintCfg {
    fn from(typescript: TypeScriptCfg) -> Self {
        DprintCfg {
            typescript,
            schema_url: Default::default(),
        }
    }
}

impl From<TypeScriptCfgWithSchema> for DprintCfg {
    fn from(typescript: TypeScriptCfgWithSchema) -> Self {
        TypeScriptCfg::from(typescript).into()
    }
}
