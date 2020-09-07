#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OutputKind {
    TypeScript,
    DprintRc,
}

impl std::str::FromStr for OutputKind {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "typescript" => OutputKind::TypeScript,
            "dprintrc" => OutputKind::DprintRc,
            _ => return Err(text.to_string()),
        })
    }
}
