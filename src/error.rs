use derive_more::From;
use pipe_trait::Pipe;
use std::io;
use thiserror::Error;

#[derive(Debug, From, Error)]
#[error("{0}")]
pub enum Error {
    TagReadFailure(TagReadFailure),
    PictureTypeNotFound(PictureTypeNotFound),
    PictureNotFound(PictureNotFound),
    NoPicTypeMultiPic(NoPicTypeMultiPic),
    DeserializationFailure(DeserializationFailure),
    PictureFileWriteFailure(PictureFileWriteFailure),
}

#[derive(Debug, From, Error)]
#[error("Failed to read tag from file: {error}")]
pub struct TagReadFailure {
    pub error: id3::Error,
}

#[derive(Debug, Error)]
#[error("Picture of type {picture_type:?} not found")]
pub struct PictureTypeNotFound {
    pub picture_type: String,
}

#[derive(Debug, Error)]
#[error("Picture not found")]
pub struct PictureNotFound;

#[derive(Debug, Error)]
#[error("Multiple pictures detected but no picture type specified")]
pub struct NoPicTypeMultiPic;

#[derive(Debug, From, Error)]
#[error("Failed to deserialize: {0}")]
pub enum DeserializationFailure {
    Json(serde_json::Error),
    Toml(toml::ser::Error),
    Yaml(serde_yaml::Error),
}

#[derive(Debug, From, Error)]
#[error("Failed to write picture to file: {error}")]
pub struct PictureFileWriteFailure {
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
indirect_convert!(toml::ser::Error, DeserializationFailure);
indirect_convert!(serde_yaml::Error, DeserializationFailure);
