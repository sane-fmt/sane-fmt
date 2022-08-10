use clap::ValueEnum;

#[derive(Debug, PartialEq, Clone, Copy, ValueEnum)]
pub enum LogFormat {
    #[clap(name = "human")]
    Human,
    #[clap(name = "github-actions")]
    GitHubActions,
}
