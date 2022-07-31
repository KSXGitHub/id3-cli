use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        Run,
    },
    error::Error,
    utils::ModifyTags,
};
use clap::{Args, Subcommand};
use id3::{Tag, TagLike};
use std::{mem::replace, path::PathBuf};

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

/// CLI arguments of `delete all` subcommand.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct DeleteAllField {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Path to the target audio file.
    pub target_audio: PathBuf,
}

impl Run for DeleteAllField {
    fn run(self) -> Result<(), Error> {
        let DeleteAllField {
            no_backup,
            target_audio,
        } = self;
        ModifyTags::builder()
            .no_backup(no_backup)
            .target_audio(&target_audio)
            .build()
            .run(|tag| replace(tag, Tag::new()))?;
        Ok(())
    }
}

/// Single-field subcommand of the `delete` subcommand.
pub type DeleteSingleField = Field<DeleteArgsTable>;

impl Run for Text<DeleteArgsTable> {
    fn run(self) -> Result<(), Error> {
        fn delete_text(args: DeleteText, delete: impl FnOnce(&mut Tag)) -> Result<(), Error> {
            let DeleteText {
                no_backup,
                target_audio,
            } = args;
            ModifyTags::builder()
                .no_backup(no_backup)
                .target_audio(&target_audio)
                .build()
                .run(delete)
        }

        match self {
            Text::Title(args) => delete_text(args, Tag::remove_title),
            Text::Artist(args) => delete_text(args, Tag::remove_artist),
            Text::Album(args) => delete_text(args, Tag::remove_album),
            Text::AlbumArtist(args) => delete_text(args, Tag::remove_album_artist),
            Text::Genre(args) => delete_text(args, Tag::remove_genre),
            Text::GenreCode(args) => delete_text(args, Tag::remove_genre),
        }
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

/// CLI arguments of `delete <text-field>`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct DeleteText {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Path to the target audio file.
    pub target_audio: PathBuf,
}

/// CLI arguments of `delete comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct DeleteComment {}

impl Run for DeleteComment {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}

/// CLI arguments of `delete picture`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct DeletePicture {}

impl Run for DeletePicture {
    fn run(self) -> Result<(), Error> {
        todo!()
    }
}
