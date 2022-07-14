use id3::{self, Tag};
use mediatype::{
    names::{BMP, GIF, IMAGE, JPEG, PNG, SVG, WEBP},
    MediaType,
};
use pipe_trait::Pipe;
use std::path::Path;

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
pub(crate) fn read_tag_from_path(path: impl AsRef<Path>) -> id3::Result<Tag> {
    path.pipe(Tag::read_from_path).or_else(no_tag_to_empty_tag)
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
