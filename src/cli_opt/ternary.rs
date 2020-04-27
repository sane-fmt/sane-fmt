#[derive(PartialEq, Debug)]
pub enum Ternary {
    Auto,
    Never,
    Always,
}

impl std::str::FromStr for Ternary {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "auto" => Ternary::Auto,
            "never" => Ternary::Never,
            "always" => Ternary::Always,
            _ => Err(text.to_owned())?,
        })
    }
}
