use crate::{
    backup::Backup,
    error::{Error, FileReadFailure, TagReadFailure, TagWriteFailure},
};
use id3::{self, Tag};
use mediatype::{
    names::{BMP, GIF, IMAGE, JPEG, PNG, SVG, WEBP},
    MediaType,
};
use pipe_trait::Pipe;
use sha2::{Digest, Sha256};
use std::{fmt::Debug, fs::read as read_file, path::Path};
use typed_builder::TypedBuilder;

/// Return an empty tag if the reason for the error was "no tag".
/// Otherwise, return the error.
pub(crate) fn no_tag_to_empty_tag(error: id3::Error) -> id3::Result<Tag> {
    if matches!(error.kind, id3::ErrorKind::NoTag) {
        Ok(Tag::new())
    } else {
        Err(error)
    }
}

/// Read tag from a path.
/// Return an empty tag if tag does not exist.
pub fn read_tag_from_path(path: impl AsRef<Path>) -> Result<Tag, Error> {
    path.pipe_as_ref(read_file)
        .map_err(|error| FileReadFailure {
            file: path.as_ref().to_path_buf(),
            error,
        })?
        .pipe(read_tag_from_data)
        .map_err(TagReadFailure::from)
        .map_err(Error::from)
}

/// Read tag from a binary blob.
/// Return an empty tag if tag does not exist.
pub(crate) fn read_tag_from_data(data: impl AsRef<[u8]>) -> id3::Result<Tag> {
    data.pipe_as_ref(Tag::read_from)
        .or_else(no_tag_to_empty_tag)
}

/// Get extension from some well known image media types.
pub(crate) fn get_image_extension(mime: MediaType) -> Option<&str> {
    if mime.ty != IMAGE {
        return None;
    }

    macro_rules! map_subty_ext {
        ($subty:ident -> $ext:literal) => {
            if mime.subty.as_str() == $subty.as_str() {
                return Some($ext);
            }
        };
    }

    map_subty_ext!(BMP -> "bmp");
    map_subty_ext!(GIF -> "gif");
    map_subty_ext!(JPEG -> "jpg");
    map_subty_ext!(PNG -> "png");
    map_subty_ext!(SVG -> "svg");
    map_subty_ext!(WEBP -> "webp");

    None
}

/// Create sha256 hash of a binary blob.
pub fn sha256_data(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Modify id3 tags of an audio file.
#[derive(Debug, Clone, Copy, TypedBuilder)]
pub(crate) struct ModifyTags<'a> {
    /// Whether `--no-backup` was specified.
    pub no_backup: bool,
    /// Provided `<TARGET_AUDIO>` argument.
    pub target_audio: &'a Path,
}

impl<'a> ModifyTags<'a> {
    /// Run a callback that modify the tags of an audio file.
    pub fn run<Callback, Value>(self, callback: Callback) -> Result<Value, Error>
    where
        Callback: FnOnce(&mut Tag) -> Value,
    {
        let ModifyTags {
            no_backup,
            target_audio,
        } = self;
        let audio_content = read_file(&target_audio).map_err(|error| FileReadFailure {
            file: target_audio.to_path_buf(),
            error,
        })?;
        if !no_backup {
            Backup::builder()
                .source_file_path(target_audio)
                .source_file_hash(&sha256_data(&audio_content))
                .build()
                .backup()?;
        }
        let mut tag = read_tag_from_data(&audio_content).map_err(TagReadFailure::from)?;
        let version = tag.version();

        let value = callback(&mut tag);

        tag.write_to_path(target_audio, version)
            .map_err(TagWriteFailure::from)?;

        Ok(value)
    }
}
