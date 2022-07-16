use chrono::{DateTime, Datelike, Local, Timelike};
use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;

/// Parameters to construct a backup file path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TypedBuilder)]
pub struct FilePath<'a> {
    /// Name of the source file (not path).
    pub source_file_name: &'a Path,
    /// Hash of the source file in hexadecimal string.
    pub source_file_hash: &'a str,
    /// Current date time.
    #[builder(default = Local::now())]
    pub date_time: DateTime<Local>,
}

impl<'a> FilePath<'a> {
    /// Construct backup file path.
    pub fn path(self) -> PathBuf {
        let FilePath {
            source_file_name,
            source_file_hash,
            date_time,
        } = self;
        let date = date_time.date();
        let date = format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day());
        let time = date_time.time();
        let time = format!(
            "{:02}.{:02}.{:02}",
            time.hour(),
            time.minute(),
            time.second(),
        );
        Path::new(".id3-backups")
            .join(source_file_name)
            .join(date)
            .join(time)
            .join(source_file_hash)
    }
}

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use super::FilePath;
    use chrono::{Local, TimeZone};
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn file_path() {
        let received = FilePath::builder()
            .source_file_name(Path::new("mysterious-file.mp3"))
            .source_file_hash("34a1e24aba0a02316b786933761beedcea40c8eda46a39054f994e0fdef87adf")
            .date_time(Local.ymd(2022, 7, 16).and_hms(12, 26, 5))
            .build()
            .path();
        let expected = Path::new(".id3-backups")
            .join("mysterious-file.mp3")
            .join("2022-07-16")
            .join("12.26.05")
            .join("34a1e24aba0a02316b786933761beedcea40c8eda46a39054f994e0fdef87adf");
        assert_eq!(received, expected);
    }
}
