use crate::args::{
    field::{ArgsTable, Field},
    text_format::TextFormat,
};
use clap::{Args, Subcommand};
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
    /// Format of the output text.
    #[clap(long, value_enum, default_value = "plain")]
    pub format: TextFormat,
}

/// CLI arguments of `view picture`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct PictureViewArgs {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub command: PictureViewCmd,
}

/// Subcommand of `view picture`.
#[derive(Debug, Subcommand)]
#[clap(about = "")]
pub enum PictureViewCmd {
    /// List descriptions, mime types, picture types, and sizes of all pictures.
    List(PictureListArgs),
    /// Export a single picture to a file.
    File(PictureFileArgs),
}

/// CLI arguments of `view picture file`.
#[derive(Debug, Args)]
pub struct PictureListArgs {
    /// Format of the output text.
    #[clap(long, value_enum, default_value = "plain")]
    pub format: TextFormat,
}

/// CLI arguments of `view picture file`.
#[derive(Debug, Args)]
pub struct PictureFileArgs {
    /// Picture type to export. Required when there are multiple pictures.
    #[clap(long, short = 't')]
    pub picture_type: Option<String>,
    /// Path to the output file.
    pub output: PathBuf,
}
