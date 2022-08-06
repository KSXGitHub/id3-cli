use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        picture_type::PictureType,
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
            Text::Genre(DeleteGenre::Genre(args)) => delete_text(args, Tag::remove_genre),
        }
    }
}

/// Table of [`Args`] types for [`Delete`].
#[derive(Debug)]
pub struct DeleteArgsTable;
impl ArgsTable for DeleteArgsTable {
    type Text = DeleteText;
    type Genre = DeleteGenre;
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

/// Genre fields.
#[derive(Debug, Subcommand)]
pub enum DeleteGenre {
    #[clap(name = "genre")]
    Genre(DeleteText),
}

/// CLI arguments of `delete comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct DeleteComment {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Comment description.
    #[clap(long)]
    pub description: Option<String>,
    /// Comment content.
    #[clap(long)]
    pub content: Option<String>,
    /// Path to the target audio file.
    pub target_audio: PathBuf,
}

impl Run for DeleteComment {
    fn run(self) -> Result<(), Error> {
        let DeleteComment {
            no_backup,
            ref description,
            ref content,
            ref target_audio,
        } = self;
        let description = description.as_deref();
        let content = content.as_deref();
        ModifyTags::builder()
            .no_backup(no_backup)
            .target_audio(target_audio)
            .build()
            .run(|tag| tag.remove_comment(description, content))
    }
}

/// CLI arguments of `delete picture`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct DeletePicture {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Path to the target audio file.
    pub target_audio: PathBuf,
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub command: DeletePictureCmd,
}

impl Run for DeletePicture {
    fn run(self) -> Result<(), Error> {
        let DeletePicture {
            no_backup,
            ref target_audio,
            command,
        } = self;

        ModifyTags {
            no_backup,
            target_audio,
        }
        .run(|tag| match command {
            DeletePictureCmd::All => tag.remove_all_pictures(),
            DeletePictureCmd::ByType(picture_type) => {
                tag.remove_picture_by_type(picture_type.into())
            }
        })
    }
}

/// Subcommand of `delete picture`.
#[derive(Debug, Subcommand)]
#[clap(about = "")]
pub enum DeletePictureCmd {
    /// Delete all pictures.
    All,
    /// Delete a picture by type.
    #[clap(flatten)]
    ByType(PictureType),
}
