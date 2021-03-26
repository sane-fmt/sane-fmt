use serde::Serialize;

const SCHEMA_URL: &str = "https://dprint.dev/schemas/v0.json";

#[derive(Debug, Default, Serialize, Clone)]
#[serde(into = "&str")]
pub struct SchemaUrl;

impl From<SchemaUrl> for &str {
    fn from(_: SchemaUrl) -> Self {
        SCHEMA_URL
    }
}
