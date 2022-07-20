use crate::{
    app::field::{ArgsTable, Field, Text},
    error::{
        AmbiguousCommentChoices, AmbiguousPictureChoices, CommentNotFound, Error,
        OutputDirCreationFailure, PictureFileWriteFailure, PictureIdOutOfBound, PictureNotFound,
    },
    run::Run,
    text_data::{comment::Comment, picture::Picture},
    text_format::TextFormat,
    utils::{get_image_extension, read_tag_from_path},
};
use clap::{Args, Subcommand};
use id3::{Tag, TagLike};
use mediatype::MediaType;
use pipe_trait::Pipe;
use std::{borrow::Cow, fs, path::PathBuf};

/// Subcommand of the `get` subcommand.
pub type Get = Field<GetArgsTable>;

impl Run for Text<GetArgsTable> {
    fn run(self) -> Result<(), Error> {
        macro_rules! get_text {
            ($args:expr, $get:expr) => {{
                let GetText {
                    format,
                    input_audio,
                } = $args;
                let tag = read_tag_from_path(input_audio)?;
                let value = $get(&tag);
                match (format, value) {
                    (Some(format), value) => println!("{}", format.serialize(&value)?),
                    (None, Some(value)) => println!("{value}"),
                    (None, None) => {}
                }
                Ok(())
            }};
        }

        match self {
            Text::Title(args) => get_text!(args, Tag::title),
            Text::Artist(args) => get_text!(args, Tag::artist),
            Text::Album(args) => get_text!(args, Tag::album),
            Text::AlbumArtist(args) => get_text!(args, Tag::album_artist),
            Text::Genre(args) => get_text!(args, Tag::genre_parsed),
        }
    }
}

/// Table of [`Args`] types for [`Get`].
#[derive(Debug)]
pub struct GetArgsTable;
impl ArgsTable for GetArgsTable {
    type Text = GetText;
    type Comment = GetComment;
    type Picture = GetPicture;
}

/// CLI arguments of `get <text-field>`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct GetText {
    /// Format the output text into JSON or YAML.
    #[clap(long, value_enum)]
    pub format: Option<TextFormat>,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
}

/// CLI arguments of `get comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct GetComment {
    /// Filter language.
    #[clap(long)]
    pub language: Option<String>,
    /// Filter description.
    #[clap(long)]
    pub description: Option<String>,
    /// Format of the output text. Required if there are multiple comments.
    #[clap(long, value_enum)]
    pub format: Option<TextFormat>,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
}

impl Run for GetComment {
    fn run(self) -> Result<(), Error> {
        let GetComment {
            language,
            description,
            format,
            input_audio,
        } = self;
        let tag = read_tag_from_path(input_audio)?;
        let comments = tag
            .comments()
            .filter(|comment| {
                language
                    .as_ref()
                    .map_or(true, |language| &comment.lang == language)
            })
            .filter(|comment| {
                description
                    .as_ref()
                    .map_or(true, |description| &comment.description == description)
            });
        let output_text: Cow<str> = if let Some(format) = format {
            let comments: Vec<_> = comments.map(Comment::from).collect();
            format.serialize(&comments)?.pipe(Cow::Owned)
        } else {
            let mut iter = comments;
            let comment = iter.next().ok_or(CommentNotFound)?;
            if iter.next().is_some() {
                return AmbiguousCommentChoices.pipe(Error::from).pipe(Err);
            }
            Cow::Borrowed(&comment.text)
        };
        println!("{output_text}");
        Ok(())
    }
}

/// CLI arguments of `get picture`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct GetPicture {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub command: GetPictureCmd,
}

impl Run for GetPicture {
    fn run(self) -> Result<(), Error> {
        self.command.run()
    }
}

/// Subcommand of `get picture`.
#[derive(Debug, Subcommand)]
#[clap(about = "")]
pub enum GetPictureCmd {
    /// List descriptions, mime types, picture types, and sizes of all pictures.
    List(GetPictureList),
    /// Export a single picture to a file.
    File(GetPictureFile),
    /// Export all single pictures to a directory.
    Dir(GetPictureDir),
}

impl Run for GetPictureCmd {
    fn run(self) -> Result<(), Error> {
        match self {
            GetPictureCmd::List(proc) => proc.run(),
            GetPictureCmd::File(proc) => proc.run(),
            GetPictureCmd::Dir(proc) => proc.run(),
        }
    }
}

/// CLI arguments of `get picture file`.
#[derive(Debug, Args)]
pub struct GetPictureList {
    /// Format of the output text.
    #[clap(long, value_enum)]
    pub format: TextFormat,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
}

impl Run for GetPictureList {
    fn run(self) -> Result<(), Error> {
        let GetPictureList {
            format,
            input_audio,
        } = self;
        let tag = read_tag_from_path(input_audio)?;
        let pictures: Vec<_> = tag
            .pictures()
            .map(Picture::from_id3_ref)
            .zip(0..)
            .map(|(picture, id)| picture.with_id(id))
            .collect();
        let serialized = format.serialize(&pictures)?;
        println!("{serialized}");
        Ok(())
    }
}

/// CLI arguments of `get picture file`.
#[derive(Debug, Args)]
pub struct GetPictureFile {
    /// Picture ID to export. Required if there are multiple pictures.
    #[clap(long)]
    pub id: Option<u16>,
    /// Path to the input audio file.
    pub input_audio: PathBuf,
    /// Path to the output picture file.
    pub output_picture: PathBuf,
}

impl Run for GetPictureFile {
    fn run(self) -> Result<(), Error> {
        let GetPictureFile {
            id,
            input_audio,
            output_picture,
        } = self;
        let tag = read_tag_from_path(input_audio)?;
        let data = if let Some(target_id) = id {
            &tag.pictures()
                .zip(0u16..)
                .find(|(_, current_id)| *current_id == target_id)
                .ok_or(PictureIdOutOfBound)?
                .0
                .data
        } else {
            let mut iter = tag.pictures().map(|picture| &picture.data);
            let data = iter.next().ok_or(PictureNotFound)?;
            if iter.next().is_some() {
                return AmbiguousPictureChoices.pipe(Error::from).pipe(Err);
            }
            data
        };
        fs::write(output_picture, data)
            .map_err(PictureFileWriteFailure::from)
            .map_err(Error::from)
    }
}

/// CLI arguments of `get picture dir`.
#[derive(Debug, Args)]
pub struct GetPictureDir {
    /// Path to the input audio file.
    pub input_audio: PathBuf,
    /// Path to the directory to contain the output pictures.
    pub output_directory: PathBuf,
}

impl Run for GetPictureDir {
    fn run(self) -> Result<(), Error> {
        let GetPictureDir {
            input_audio,
            output_directory,
        } = self;

        let tag = read_tag_from_path(input_audio)?;
        let pictures = tag.pictures().zip(0..);
        for (picture, index) in pictures {
            let id3::frame::Picture {
                picture_type,
                mime_type,
                description,
                data,
            } = picture;
            fs::create_dir_all(&output_directory).map_err(OutputDirCreationFailure::from)?;
            eprintln!("#{index}: {picture_type} {mime_type} {description}");
            let ext = MediaType::parse(mime_type)
                .ok()
                .and_then(get_image_extension);
            let output_file_name = match ext {
                Some(ext) => format!("{index}.{ext}"),
                None => index.to_string(),
            };
            let output_file_path = output_directory.join(output_file_name);
            fs::write(output_file_path, data).map_err(PictureFileWriteFailure::from)?;
        }

        Ok(())
    }
}
