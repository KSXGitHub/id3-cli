use crate::args::field::Field;
use clap::Args;
use std::path::PathBuf;

/// Subcommand of the `view` subcommand.
pub type ViewCmd = Field<TextViewArgs, FrameViewArgs>;

/// CLI arguments of `view <text-field>`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct TextViewArgs {}

/// CLI arguments of `view <frame-field>`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct FrameViewArgs {
    /// Path to the output file.
    pub output: PathBuf,
}
