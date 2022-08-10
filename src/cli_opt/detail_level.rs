use clap::ValueEnum;

#[derive(Debug, PartialEq, Clone, Copy, ValueEnum)]
pub enum DetailLevel {
    Count,
    Name,
    Diff,
}
