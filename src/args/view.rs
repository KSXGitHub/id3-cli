use crate::args::field::{ArgsTable, Field};
use clap::Args;
use std::path::PathBuf;

/// Subcommand of the `view` subcommand.
pub type ViewCmd = Field<ViewArgsTable>;

#[derive(Debug)]
pub struct ViewArgsTable;
impl ArgsTable for ViewArgsTable {
    type Text = TextViewArgs;
    type Picture = FrameViewArgs;
    type Comment = FrameViewArgs;
}

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
