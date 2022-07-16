use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        Run,
    },
    error::Error,
};
use clap::Args;
use std::path::PathBuf;

/// Subcommand of the `set` subcommand.
pub type Set = Field<SetArgsTable>;

impl Run for Text<SetArgsTable> {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

/// Table of [`Args`] types for [`Set`].
#[derive(Debug)]
pub struct SetArgsTable;
impl ArgsTable for SetArgsTable {
    type Text = SetText;
    type Comment = SetComment;
    type Picture = SetPicture;
}

/// CLI arguments of `set <text-field>`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct SetText {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Path to the input audio file.
    pub target_audio: PathBuf,
    /// New value to set.
    pub value: String,
}

/// CLI arguments of `set comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct SetComment {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Path to the input audio file.
    pub target_audio: PathBuf,
    // TODO: implement
}

impl Run for SetComment {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

/// CLI arguments of `set comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct SetPicture {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Path to the input audio file.
    pub target_audio: PathBuf,
    // TODO: implement
}

impl Run for SetPicture {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}
