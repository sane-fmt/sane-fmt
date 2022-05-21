use serde::Serialize;

const DPRINT_CORE_SCHEMA_URL: &str = "https://dprint.dev/schemas/v0.json";
const DPRINT_TYPESCRIPT_SCHEMA_URL: &str =
    "https://plugins.dprint.dev/dprint/dprint-plugin-typescript/latest/schema.json";

#[derive(Debug, Default, Serialize, Clone)]
#[serde(into = "&str")]
pub struct DprintCoreSchemaUrl;

impl From<DprintCoreSchemaUrl> for &str {
    fn from(_: DprintCoreSchemaUrl) -> Self {
        DPRINT_CORE_SCHEMA_URL
    }
}

#[derive(Debug, Default, Serialize, Clone)]
#[serde(into = "&str")]
pub struct DprintTypeScriptSchemaUrl;

impl From<DprintTypeScriptSchemaUrl> for &str {
    fn from(_: DprintTypeScriptSchemaUrl) -> Self {
        DPRINT_TYPESCRIPT_SCHEMA_URL
    }
}
