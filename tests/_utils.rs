use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command, Output as CommandOutput, Stdio},
    str::from_utf8,
};

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
pub struct Exe {
    /// Command the execute.
    pub cmd: Command,
    /// Working directory.
    pub wdir: PathBuf,
}

impl Exe {
    /// Create a wrapper with specified working directory.
    pub fn new<Dir: AsRef<OsStr> + ?Sized>(wdir_ref: &Dir) -> Self {
        let mut cmd = Command::new(EXE);
        let wdir = Path::new(wdir_ref).to_path_buf();
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

/// Convert `[u8]` to `String`.
pub fn u8v_to_string(u8v: &[u8]) -> &str {
    from_utf8(u8v).expect("convert [u8] to String")
}
