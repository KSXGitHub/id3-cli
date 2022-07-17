use derive_more::{AsRef, Deref};
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::{
    env::temp_dir,
    ffi::OsStr,
    fs::remove_dir_all,
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command, Output as CommandOutput, Stdio},
    str::from_utf8,
};
use tempfile as tmp;

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
#[derive(Debug, PartialEq, Eq, Clone, AsRef, Deref)]
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
        remove_dir_all(target).expect("delete temporary directory");
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
