use crate::error::InvalidFilePath;
use chrono::{DateTime, Datelike, Local, Timelike};
use pipe_trait::Pipe;
use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;

/// Parameters to construct a backup file path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TypedBuilder)]
pub struct FilePath<'a> {
    /// Path to the source file.
    pub source_file_path: &'a Path,
    /// Hash of the source file in hexadecimal string.
    pub source_file_hash: &'a str,
    /// Current date time.
    #[builder(default = Local::now())]
    pub date_time: DateTime<Local>,
}

impl<'a> FilePath<'a> {
    /// Construct backup file path.
    pub fn path(self) -> Result<PathBuf, InvalidFilePath> {
        let FilePath {
            source_file_path,
            source_file_hash,
            date_time,
        } = self;
        let source_file_parent = source_file_path.parent().ok_or(InvalidFilePath)?;
        let source_file_name = source_file_path.file_name().ok_or(InvalidFilePath)?;
        let date = date_time.date();
        let date = format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day());
        let time = date_time.time();
        let time = format!(
            "{:02}.{:02}.{:02}",
            time.hour(),
            time.minute(),
            time.second(),
        );
        source_file_parent
            .join(".id3-backups")
            .join(source_file_name)
            .join(date)
            .join(time)
            .join(source_file_hash)
            .pipe(Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::FilePath;
    use chrono::{Local, TimeZone};
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn file_path() {
        let source_file_parent = Path::new("Music").join("fav");
        let source_file_name = "mysterious-file.mp3";
        let source_file_path = source_file_parent.join(source_file_name);
        let received = FilePath::builder()
            .source_file_path(&source_file_path)
            .source_file_hash("34a1e24aba0a02316b786933761beedcea40c8eda46a39054f994e0fdef87adf")
            .date_time(Local.ymd(2022, 7, 16).and_hms(12, 26, 5))
            .build()
            .path()
            .expect("get backup file path");
        let expected = source_file_parent
            .join(".id3-backups")
            .join(source_file_name)
            .join("2022-07-16")
            .join("12.26.05")
            .join("34a1e24aba0a02316b786933761beedcea40c8eda46a39054f994e0fdef87adf");
        assert_eq!(received, expected);
    }
}
