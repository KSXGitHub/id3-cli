use crate::args::field::{ArgsTable, Field};
use clap::Args;
use std::path::PathBuf;

/// Subcommand of the `view` subcommand.
pub type ViewCmd = Field<ViewArgsTable>;

/// Table of [`Args`] types for [`ViewCmd`].
#[derive(Debug)]
pub struct ViewArgsTable;
impl ArgsTable for ViewArgsTable {
    type Text = TextViewArgs;
    type Comment = CommentViewArgs;
    type Picture = PictureViewArgs;
}

/// CLI arguments of `view <text-field>`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct TextViewArgs {}

/// CLI arguments of `view comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct CommentViewArgs {
    /// Filter language.
    #[clap(long)]
    pub lang: Option<String>,
    /// Filter description.
    #[clap(long)]
    pub description: Option<String>,
}

/// CLI arguments of `view picture`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct PictureViewArgs {
    /// Filter description.
    #[clap(long)]
    pub description: Option<String>,
    /// Filter MIME type.
    #[clap(long)]
    pub mime_type: Option<String>,
    /// Filter picture type.
    #[clap(long)]
    pub picture_type: Option<String>,
    /// Path to the output file.
    pub output: PathBuf,
}
