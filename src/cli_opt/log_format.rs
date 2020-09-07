#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogFormat {
    Human,
    GitHubActions,
}

impl std::str::FromStr for LogFormat {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "human" => LogFormat::Human,
            "github-actions" => LogFormat::GitHubActions,
            _ => return Err(text.to_string()),
        })
    }
}
