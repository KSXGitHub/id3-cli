use crate::error::{Error, TagReadFailure};
use id3::{self, Tag};
use mediatype::{
    names::{BMP, GIF, IMAGE, JPEG, PNG, SVG, WEBP},
    MediaType,
};
use pipe_trait::Pipe;
use sha2::{Digest, Sha256};
use std::{fmt::Debug, fs::read as read_file, path::Path};

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
pub(crate) fn read_tag_from_path(path: impl AsRef<Path>) -> Result<Tag, Error> {
    path.pipe(Tag::read_from_path)
        .or_else(no_tag_to_empty_tag)
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

/// Create sha256 hash of a file.
pub fn sha256_file(file_name: impl AsRef<Path> + Debug) -> String {
    read_file(&file_name)
        .unwrap_or_else(|error| panic!("Failed to read {file_name:?}: {error}"))
        .pipe(sha256_data)
}
