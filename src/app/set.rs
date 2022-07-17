use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        Run,
    },
    backup::Backup,
    error::{Error, FileReadFailure, TagWriteFailure},
    utils::{read_tag_from_data, sha256_data},
};
use clap::Args;
use id3::{Tag, TagLike};
use std::{fs::read as read_file, path::PathBuf};

/// Subcommand of the `set` subcommand.
pub type Set = Field<SetArgsTable>;

impl Run for Text<SetArgsTable> {
    fn run(self) -> Result<(), Error> {
        fn set_text(args: SetText, set: impl FnOnce(&mut Tag, String)) -> Result<(), Error> {
            let SetText {
                no_backup,
                target_audio,
                value,
            } = args;
            let audio_content = read_file(&target_audio).map_err(|error| FileReadFailure {
                file: target_audio.clone(),
                error,
            })?;
            if !no_backup {
                Backup::builder()
                    .source_file_path(&target_audio)
                    .source_file_hash(&sha256_data(&audio_content))
                    .build()
                    .backup()?;
            }
            let mut tag = read_tag_from_data(&audio_content)?;
            let version = tag.version();
            set(&mut tag, value);
            tag.write_to_path(target_audio, version)
                .map_err(TagWriteFailure::from)?;
            Ok(())
        }

        match self {
            Text::Title(args) => set_text(args, Tag::set_title),
            Text::Artist(args) => set_text(args, Tag::set_artist),
            Text::Album(args) => set_text(args, Tag::set_album),
            Text::AlbumArtist(args) => set_text(args, Tag::set_album_artist),
            Text::Genre(args) => set_text(args, |_, _| unimplemented!()),
        }
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
