use crate::{
    args::field::{ArgsTable, Field, Text},
    error::{
        Error, NoPicTypeMultiPic, PictureFileWriteFailure, PictureNotFound, PictureTypeNotFound,
    },
    run::Run,
    text_data::picture::Picture,
    text_format::TextFormat,
};
use clap::{Args, Subcommand};
use id3::{Tag, TagLike};
use pipe_trait::Pipe;
use serde_json::json;
use std::{fs, path::PathBuf};

/// Subcommand of the `view` subcommand.
pub type ViewCmd = Field<ViewArgsTable>;

impl Run for Text<ViewArgsTable> {
    fn run(self) -> Result<(), Error> {
        match self {
            Text::Title(args) => view_text(args, Tag::title),
            Text::Artist(args) => view_text(args, Tag::artist),
            Text::Album(args) => view_text(args, Tag::album),
            Text::AlbumArtist(args) => view_text(args, Tag::album_artist),
            Text::Genre(args) => view_text(args, Tag::genre),
        }
    }
}

fn view_text(args: TextViewArgs, get: impl FnOnce(&Tag) -> Option<&str>) -> Result<(), Error> {
    let TextViewArgs { input_audio } = args;
    let tag = Tag::read_from_path(input_audio)?;
    if let Some(title) = get(&tag) {
        println!("{title}");
    }
    Ok(())
}

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
pub struct TextViewArgs {
    /// Path to the input file.
    pub input_audio: PathBuf,
}

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
    #[clap(long, value_enum, default_value = "json")]
    pub format: TextFormat,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
}

impl Run for CommentViewArgs {
    fn run(self) -> Result<(), Error> {
        let CommentViewArgs {
            lang,
            description,
            format,
            input_audio,
        } = self;
        let tag = Tag::read_from_path(input_audio)?;
        let comments: Vec<_> = tag
            .comments()
            .filter(|comment| lang.as_ref().map_or(true, |lang| &comment.lang == lang))
            .filter(|comment| {
                description
                    .as_ref()
                    .map_or(true, |description| &comment.description == description)
            })
            .map(|comment| {
                json!({
                    "lang": comment.lang,
                    "description": comment.description,
                    "text": comment.text,
                })
            })
            .collect();
        let serialized = format.serialize(&comments)?;
        println!("{serialized}");
        Ok(())
    }
}

/// CLI arguments of `view picture`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct PictureViewArgs {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub command: PictureViewCmd,
}

impl Run for PictureViewArgs {
    fn run(self) -> Result<(), Error> {
        self.command.run()
    }
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

impl Run for PictureViewCmd {
    fn run(self) -> Result<(), Error> {
        match self {
            PictureViewCmd::List(args) => args.run(),
            PictureViewCmd::File(args) => args.run(),
        }
    }
}

/// CLI arguments of `view picture file`.
#[derive(Debug, Args)]
pub struct PictureListArgs {
    /// Format of the output text.
    #[clap(long, value_enum, default_value = "json")]
    pub format: TextFormat,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
}

impl Run for PictureListArgs {
    fn run(self) -> Result<(), Error> {
        let PictureListArgs {
            format,
            input_audio,
        } = self;
        let tag = Tag::read_from_path(input_audio)?;
        let pictures: Vec<_> = tag.pictures().map(Picture::from_id3_ref).collect();
        let serialized = format.serialize(&pictures)?;
        println!("{serialized}");
        Ok(())
    }
}

/// CLI arguments of `view picture file`.
#[derive(Debug, Args)]
pub struct PictureFileArgs {
    /// Picture type to export. Required when there are multiple pictures.
    #[clap(long, short = 't')]
    pub picture_type: Option<String>,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
    /// Path to the output picture file.
    pub output_picture: PathBuf,
}

impl Run for PictureFileArgs {
    fn run(self) -> Result<(), Error> {
        let PictureFileArgs {
            picture_type,
            input_audio,
            output_picture,
        } = self;
        let tag = Tag::read_from_path(input_audio)?;
        let data = if let Some(picture_type) = picture_type {
            let lowercase_picture_type = picture_type.to_lowercase();
            &tag.pictures()
                .find(|picture| {
                    picture.picture_type.to_string().to_lowercase() == lowercase_picture_type
                })
                .ok_or(PictureTypeNotFound { picture_type })?
                .data
        } else {
            let mut iter = tag.pictures().map(|picture| &picture.data);
            let data = iter.next().ok_or(PictureNotFound)?;
            if iter.next().is_some() {
                return NoPicTypeMultiPic.pipe(Error::from).pipe(Err);
            }
            data
        };
        fs::write(output_picture, data)
            .map_err(PictureFileWriteFailure::from)
            .map_err(Error::from)
    }
}
