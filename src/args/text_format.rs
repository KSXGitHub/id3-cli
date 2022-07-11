use clap::ValueEnum;

/// Format of input/output text values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(about = "")]
pub enum TextFormat {
    Plain,
    Json,
    Yaml,
    Toml,
}
