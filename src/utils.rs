use id3::{self, Tag};
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
