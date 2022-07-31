use crate::{
    app::{
        field::{ArgsTable, Field, Text},
        picture_type::PictureType,
        Run,
    },
    error::{Error, FileReadFailure},
    text_data::comment::Comment,
    text_format::TextFormat,
    utils::ModifyTags,
};
use clap::{Args, Subcommand};
use id3::{frame, Content, Tag, TagLike};
use pipe_trait::Pipe;
use std::{borrow::Cow, fs::read as read_file, path::PathBuf};

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
            ModifyTags::builder()
                .no_backup(no_backup)
                .target_audio(&target_audio)
                .build()
                .run(move |tag| set(tag, value))
        }

        match self {
            Text::Title(args) => set_text(args, Tag::set_title),
            Text::Artist(args) => set_text(args, Tag::set_artist),
            Text::Album(args) => set_text(args, Tag::set_album),
            Text::AlbumArtist(args) => set_text(args, Tag::set_album_artist),
            Text::Genre(SetGenre::Code(args)) => set_text(args, Tag::set_genre),
        }
    }
}

/// Table of [`Args`] types for [`Set`].
#[derive(Debug)]
pub struct SetArgsTable;
impl ArgsTable for SetArgsTable {
    type Text = SetText;
    type Genre = SetGenre;
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
    /// Path to the target audio file.
    pub target_audio: PathBuf,
    /// New value to set.
    pub value: String,
}

/// Genre fields.
#[derive(Debug, Subcommand)]
pub enum SetGenre {
    #[clap(name = "genre-code")]
    Code(SetText),
}

/// CLI arguments of `set comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct SetComment {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Comment language (ISO 639-2).
    #[clap(long)]
    pub language: Option<String>,
    /// Comment description.
    #[clap(long)]
    pub description: Option<String>,
    /// Format of the ejected comment (if any).
    #[clap(long, value_enum)]
    pub format: Option<TextFormat>,
    /// Path to the target audio file.
    pub target_audio: PathBuf,
    /// Content of the comment.
    pub content: String,
}

impl Run for SetComment {
    fn run(self) -> Result<(), Error> {
        let SetComment {
            no_backup,
            language,
            description,
            format,
            ref target_audio,
            content,
        } = self;
        let language = language.unwrap_or_else(|| "\0\0\0".to_string());
        let description = description.unwrap_or_default();

        let ejected_frame = ModifyTags {
            no_backup,
            target_audio,
        }
        .run(|tag| {
            tag.add_frame(Comment {
                language,
                description,
                content,
            })
        })?;

        let ejected_comment = match ejected_frame.as_ref().map(id3::Frame::content) {
            None => return Ok(()),
            Some(Content::Comment(comment)) => comment,
            Some(content) => panic!("Impossible! The ejected frame wasn't a comment: {content:?}"),
        };

        let output_text: Cow<str> = if let Some(format) = format {
            let output_object = Comment::from(ejected_comment);
            format.serialize(&output_object)?.pipe(Cow::Owned)
        } else {
            Cow::Borrowed(&ejected_comment.text)
        };

        println!("{output_text}");
        Ok(())
    }
}

/// CLI arguments of `set comment`.
#[derive(Debug, Args)]
#[clap(about = "")]
pub struct SetPicture {
    /// Don't create backup for the target audio file.
    #[clap(long)]
    pub no_backup: bool,
    /// Mime type of the picture.
    #[clap(long)]
    pub mime_type: Option<String>,
    /// Description of the picture.
    #[clap(long)]
    pub description: Option<String>,
    /// Path to the input audio file.
    pub target_audio: PathBuf,
    /// Type of picture.
    #[clap(value_enum)]
    pub picture_type: PictureType,
    /// Path to the input picture file.
    pub target_picture: PathBuf,
}

impl Run for SetPicture {
    fn run(self) -> Result<(), Error> {
        let SetPicture {
            no_backup,
            mime_type,
            description,
            ref target_audio,
            picture_type,
            ref target_picture,
        } = self;

        let data = read_file(target_picture).map_err(|error| FileReadFailure {
            file: target_picture.to_path_buf(),
            error,
        })?;

        let mime_type = mime_type
            .or_else(|| infer::get(&data)?.mime_type().to_string().pipe(Some))
            .unwrap_or_default();

        let description = description.unwrap_or_else(|| {
            target_picture
                .file_name()
                .map(|file_name| file_name.to_string_lossy())
                .map(|file_name| file_name.to_string())
                .unwrap_or_default()
        });

        let picture_type: frame::PictureType = picture_type.into();

        let frame = frame::Picture {
            data,
            description,
            mime_type,
            picture_type,
        };

        let ejected_picture = ModifyTags::builder()
            .target_audio(target_audio)
            .no_backup(no_backup)
            .build()
            .run(|tag| tag.add_frame(frame))?;

        if ejected_picture.is_some() {
            eprintln!("A picture had been replaced");
        } else {
            eprintln!("A picture had been added");
        }

        Ok(())
    }
}
