use super::{schema_url, TypeScriptCfg};
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeScriptCfgWithSchema {
    #[serde(rename = "$schema")]
    schema_url: schema_url::DprintTypeScriptSchemaUrl,
    #[serde(flatten)]
    config: TypeScriptCfg,
}

impl From<TypeScriptCfg> for TypeScriptCfgWithSchema {
    fn from(config: TypeScriptCfg) -> Self {
        TypeScriptCfgWithSchema {
            config,
            schema_url: Default::default(),
        }
    }
}
