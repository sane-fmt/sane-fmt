pub mod schema_url;

pub use dprint_plugin_typescript::configuration::Configuration as TypeScriptCfg;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DprintCfg {
    #[serde(rename = "$schema")]
    pub schema_url: schema_url::SchemaUrl,

    pub typescript: TypeScriptCfg,
}

impl From<TypeScriptCfg> for DprintCfg {
    fn from(typescript: TypeScriptCfg) -> Self {
        DprintCfg {
            typescript,
            schema_url: Default::default(),
        }
    }
}
