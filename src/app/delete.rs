use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        Run,
    },
    error::Error,
};
use clap::{Args, Subcommand};

/// Subcommand of the `delete` subcommand.
#[derive(Debug, Subcommand)]
pub enum Delete {
    /// Remove the whole ID3 tag from the audio.
    All(DeleteAllField),
    /// Remove a single field.
    #[clap(flatten)]
    Single(DeleteSingleField),
}

impl Run for Delete {
    fn run(self) -> Result<(), Error> {
        match self {
            Delete::All(args) => args.run(),
            Delete::Single(args) => args.run(),
        }
    }
}

#[derive(Debug, Args)]
pub struct DeleteAllField {}

impl Run for DeleteAllField {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

/// Single-field subcommand of the `delete` subcommand.
pub type DeleteSingleField = Field<DeleteArgsTable>;

impl Run for Text<DeleteArgsTable> {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

/// Table of [`Args`] types for [`Delete`].
#[derive(Debug)]
pub struct DeleteArgsTable;
impl ArgsTable for DeleteArgsTable {
    type Text = DeleteText;
    type Comment = DeleteComment;
    type Picture = DeletePicture;
}

#[derive(Debug, Args)]
pub struct DeleteText {}

impl Run for DeleteText {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct DeleteComment {}

impl Run for DeleteComment {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct DeletePicture {}

impl Run for DeletePicture {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}
