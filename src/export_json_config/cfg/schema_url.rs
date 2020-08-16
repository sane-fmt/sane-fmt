use serde::Serialize;

const SCHEMA_URL: &str = "https://dprint.dev/schemas/v0.json";

#[derive(Debug, Default, Serialize, Clone)]
#[serde(into = "&str")]
pub struct SchemaUrl;

impl Into<&'static str> for SchemaUrl {
    fn into(self) -> &'static str {
        SCHEMA_URL
    }
}
