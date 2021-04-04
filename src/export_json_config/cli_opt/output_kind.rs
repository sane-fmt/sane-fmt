#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OutputKind {
    TypeScript,
    Dprint,
}

impl std::str::FromStr for OutputKind {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "typescript" => OutputKind::TypeScript,
            "dprint" => OutputKind::Dprint,
            _ => return Err(text.to_string()),
        })
    }
}
