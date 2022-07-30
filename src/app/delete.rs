use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        Run,
    },
    error::Error,
};
use clap::Args;

/// Subcommand of the `delete` subcommand.
pub type Delete = Field<DeleteArgsTable>;

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
