use assert_cmp::assert_op;
use chrono::{DateTime, Local, TimeZone};
use derive_more::{AsRef, Deref};
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use id3_cli::utils::{read_tag_from_path, sha256_data};
use pipe_trait::Pipe;
use std::{
    env::temp_dir,
    ffi::OsStr,
    fmt::Debug,
    fs::{read as read_file, read_dir, remove_dir_all},
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command, Output as CommandOutput, Stdio},
    str::from_utf8,
};
use tempfile as tmp;
use typed_builder::TypedBuilder;

/// Version of the package.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Name of the package.
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Description of the package.
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Path to main executable.
pub const EXE: &str = env!("CARGO_BIN_EXE_id3");

/// Path to manifest.
pub const WORKSPACE: &str = env!("CARGO_MANIFEST_DIR");

/// Prefix of temp dir.
pub const TEMP_PREFIX: &str = "id3-cli-";

/// Suffix of temp dir.
pub const TEMP_SUFFIX: &str = ".test.wdir";

/// Get path to the directory of assets.
pub fn assets() -> PathBuf {
    Path::new(WORKSPACE).join("tests").join("assets")
}

/// Wrapper of main executable.
pub struct Exe<WorkDir> {
    /// Command the execute.
    pub cmd: Command,
    /// Working directory.
    pub wdir: WorkDir,
}

impl<WorkDir> Exe<WorkDir> {
    /// Create a wrapper with specified working directory.
    pub fn new(wdir: WorkDir) -> Self
    where
        WorkDir: AsRef<Path>,
    {
        let mut cmd = Command::new(EXE);
        cmd.current_dir(&wdir);
        Self { cmd, wdir }
    }

    /// Run the command.
    pub fn run(&mut self) -> ChildProcess {
        self.cmd
            .spawn()
            .map_err(|error| format!("Failed to execute main command: {}", error))
            .unwrap()
    }

    /// Takes stdin, runs command, and returns `Output`.
    pub fn run_with_stdio(
        &mut self,
        stdin: &[u8],
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> CommandOutput {
        use std::io::Write;
        let mut child = self
            .cmd
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_mut()
            .expect("unwrap child's stdin")
            .write_all(stdin)
            .expect("write data to child's stdin");
        child.wait_with_output().expect("wait for child's output")
    }
}

impl Exe<PathBuf> {
    /// Create a wrapper with the project root as working directory.
    pub fn project_root() -> Self {
        Exe::new(WORKSPACE.into())
    }
}

/// Temporary workspace that will automatically be deleted on [drop].
#[derive(Debug, AsRef, Deref)]
#[as_ref(forward)]
pub struct TempWorkspace(PathBuf);

impl Default for TempWorkspace {
    fn default() -> Self {
        let workspace = tmp::Builder::new()
            .prefix(TEMP_PREFIX)
            .suffix(TEMP_SUFFIX)
            .tempdir()
            .expect("find temporary workspace")
            .into_path();
        copy_dir(assets(), &workspace, &CopyOptions::new())
            .expect("copy assets to the temporary workspace");
        TempWorkspace(workspace)
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let target: &Path = self;
        let parent = target.parent().expect("get parent");
        assert_eq!(parent, &temp_dir());
        let name = target.file_name().expect("get name").to_string_lossy();
        assert!(name.starts_with(TEMP_PREFIX));
        assert!(name.ends_with(TEMP_SUFFIX));
        eprintln!("TempDir: Deleting {target:?}");
        if let Err(error) = remove_dir_all(target) {
            eprintln!("warning: Failed to delete {target:?}: {error}");
        }
    }
}

impl Exe<TempWorkspace> {
    /// Create a wrapper with a temporary working directory.
    pub fn temp_workspace() -> Self {
        Exe::new(TempWorkspace::default())
    }
}

/// Convert `[u8]` to `String`.
pub fn u8v_to_string(u8v: &[u8]) -> &str {
    from_utf8(u8v).expect("convert [u8] to String")
}

/// Print stdout and stderr.
pub fn show_stdout_stderr(stdout: &[u8], stderr: &[u8]) {
    let object = serde_json::json!({
        "STDOUT": u8v_to_string(stdout),
        "STDERR": u8v_to_string(stderr),
    });
    match serialize::yaml(&object) {
        Ok(yaml) => eprintln!("{yaml}"),
        Err(error) => eprintln!("show_stdout_stderr: {error}"),
    }
}

/// Create sha256 hash of a file.
pub fn sha256_file(file_name: impl AsRef<Path> + Debug) -> String {
    read_file(&file_name)
        .unwrap_or_else(|error| panic!("Failed to read {file_name:?}: {error}"))
        .pipe(sha256_data)
}

/// Create sha256 hash of an asset file.
pub fn sha256_asset(name: &str) -> String {
    assets().join(name).pipe(sha256_file)
}

/// Deserialize JSON or YAML.
pub mod deserialize {
    pub use serde_json::from_str as json;
    pub use serde_yaml::from_str as yaml;
}

/// Serialize JSON or YAML.
pub mod serialize {
    pub use serde_json::to_string_pretty as json;
    pub use serde_yaml::to_string as yaml;
}

/// Test backup of an audio file.
#[derive(Debug, TypedBuilder)]
pub struct TestBackup<'a> {
    /// Path to the workspace.
    pub workspace: &'a Path,
    /// Name (not path) of the audio file.
    pub audio_name: &'a str,
    /// Hash of the audio file before modification.
    pub initial_hash: &'a str,
    /// Moment before the command runs.
    pub before_run: DateTime<Local>,
}

impl<'a> TestBackup<'a> {
    /// Run the test.
    pub fn test(self) {
        let TestBackup {
            workspace,
            audio_name,
            initial_hash,
            before_run,
        } = self;

        // get path to the backup file
        let backup_path: Vec<_> = workspace
            .join("assets")
            .join(".id3-backups")
            .join(audio_name)
            .pipe(read_dir)
            .expect("read backup directory")
            .flatten()
            .flat_map(|entry| entry.path().pipe(read_dir))
            .flatten()
            .flatten()
            .map(|entry| entry.path().join(&initial_hash))
            .collect();
        dbg!(&backup_path);
        assert_eq!(backup_path.len(), 1);
        let backup_path = &backup_path[0];

        // compare hash
        let backup_hash = sha256_file(backup_path);
        assert_eq!(&backup_hash, initial_hash);

        // compare date and time
        let parent = backup_path.parent().expect("get parent");
        let grandparent = parent.parent().expect("get grandparent");
        let time = parent
            .file_name()
            .expect("get time")
            .to_str()
            .expect("convert time to UTF-8");
        let date = grandparent
            .file_name()
            .expect("get date")
            .to_str()
            .expect("convert date to UTF-8");
        let dt_str = format!("{}/{}", date, time);
        let dt_fmt = "%Y-%m-%d/%H.%M.%S";
        dbg!(&dt_str);
        let backup_date_time = Local
            .datetime_from_str(&dt_str, dt_fmt)
            .expect("parse date time");
        dbg!(backup_date_time);
        let distance = backup_date_time.signed_duration_since(before_run);
        dbg!(distance);
        let seconds = distance.num_seconds();
        assert_op!(seconds <= 2);
    }
}

/// Information about a comment item in a tag.
#[derive(Debug, PartialEq, Eq)]
pub struct CommentInfo {
    pub language: String,
    pub description: String,
    pub content: String,
}

impl CommentInfo {
    /// Create a [`CommentInfo`] from an [id3](id3::frame::Comment) reference.
    fn from_id3_ref(comment: &id3::frame::Comment) -> Self {
        CommentInfo {
            language: comment.lang.clone(),
            description: comment.description.clone(),
            content: comment.text.clone(),
        }
    }

    /// List all [`CommentInfo`]s from an audio file.
    pub fn from_audio_file(audio_path: impl AsRef<Path>) -> Vec<Self> {
        audio_path
            .pipe(read_tag_from_path)
            .expect("read tag from path")
            .comments()
            .map(CommentInfo::from_id3_ref)
            .collect()
    }
}

/// Information about a picture item in a tag.
#[derive(Debug, PartialEq, Eq)]
pub struct PictureInfo {
    pub mime_type: String,
    pub picture_type: String,
    pub description: String,
    pub sha256: String,
}

impl PictureInfo {
    /// Create a [`PictureInfo`] from an [id3](id3::frame::Picture) reference.
    fn from_id3_ref(picture: &id3::frame::Picture) -> Self {
        PictureInfo {
            mime_type: picture.mime_type.clone(),
            picture_type: picture.picture_type.to_string(),
            description: picture.description.clone(),
            sha256: sha256_data(&picture.data),
        }
    }

    /// List all [`PictureInfo`]s from an audio file.
    pub fn from_audio_file(audio_path: impl AsRef<Path>) -> Vec<Self> {
        audio_path
            .pipe(read_tag_from_path)
            .expect("read tag from path")
            .pictures()
            .map(PictureInfo::from_id3_ref)
            .collect()
    }
}
