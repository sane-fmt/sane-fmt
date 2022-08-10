use clap::ValueEnum;

#[derive(PartialEq, Debug, Copy, Clone, ValueEnum)]
pub enum When {
    Auto,
    Never,
    Always,
}
