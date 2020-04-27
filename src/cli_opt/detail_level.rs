#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DetailLevel {
    Count,
    Name,
    Diff,
}

impl std::str::FromStr for DetailLevel {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "count" => DetailLevel::Count,
            "name" => DetailLevel::Name,
            "diff" => DetailLevel::Diff,
            _ => Err(text.to_owned())?,
        })
    }
}
