use clap::ValueEnum;

#[derive(PartialEq, Debug, Copy, Clone, ValueEnum)]
pub enum OutputKind {
    #[clap(name = "typescript")]
    TypeScript,
    #[clap(name = "dprint")]
    Dprint,
}
