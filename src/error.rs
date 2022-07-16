use derive_more::From;
use pipe_trait::Pipe;
use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, From, Error)]
#[error("{0}")]
pub enum Error {
    TagReadFailure(TagReadFailure),
    CommentNotFound(CommentNotFound),
    AmbiguousCommentChoices(AmbiguousCommentChoices),
    PictureIdOutOfBound(PictureIdOutOfBound),
    PictureNotFound(PictureNotFound),
    AmbiguousPictureChoices(AmbiguousPictureChoices),
    DeserializationFailure(DeserializationFailure),
    PictureFileWriteFailure(PictureFileWriteFailure),
    OutputDirCreationFailure(OutputDirCreationFailure),
    InvalidFilePath(InvalidFilePath),
    BackupFailure(BackupFailure),
}

#[derive(Debug, From, Error)]
#[error("Failed to read tag from file: {error}")]
pub struct TagReadFailure {
    #[source]
    pub error: id3::Error,
}

#[derive(Debug, Error)]
#[error("Comment not found")]
pub struct CommentNotFound;

#[derive(Debug, Error)]
#[error("Too many comments to choose from")]
pub struct AmbiguousCommentChoices;

#[derive(Debug, Error)]
#[error("Specified picture ID is out of bound")]
pub struct PictureIdOutOfBound;

#[derive(Debug, Error)]
#[error("Picture not found")]
pub struct PictureNotFound;

#[derive(Debug, Error)]
#[error("Too many pictures to choose from")]
pub struct AmbiguousPictureChoices;

#[derive(Debug, From, Error)]
#[error("Failed to deserialize: {0}")]
pub enum DeserializationFailure {
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
}

#[derive(Debug, From, Error)]
#[error("Failed to write picture to file: {error}")]
pub struct PictureFileWriteFailure {
    #[source]
    pub error: io::Error,
}

#[derive(Debug, From, Error)]
#[error("Failed to create output directory: {error}")]
pub struct OutputDirCreationFailure {
    #[source]
    pub error: io::Error,
}

#[derive(Debug, From, Error)]
#[error("Provided path is not a file")]
pub struct InvalidFilePath;

#[derive(Debug, Error)]
#[error("Failed to create a backup of {src:?} at {dest:?}: {error}")]
pub struct BackupFailure {
    pub src: PathBuf,
    pub dest: PathBuf,
    #[source]
    pub error: io::Error,
}

macro_rules! indirect_convert {
    ($source:ty, $intermediate:ident) => {
        impl From<$source> for Error {
            fn from(source: $source) -> Self {
                source.pipe($intermediate::from).into()
            }
        }
    };
}

indirect_convert!(id3::Error, TagReadFailure);
indirect_convert!(serde_json::Error, DeserializationFailure);
indirect_convert!(serde_yaml::Error, DeserializationFailure);
