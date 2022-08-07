use crate::{app::Run, error::Error, utils::ModifyTags};
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct Backup {
    /// Path to the target audio file.
    pub target_audio: PathBuf,
}

impl Run for Backup {
    fn run(self) -> Result<(), Error> {
        ModifyTags::builder()
            .no_backup(false)
            .target_audio(&self.target_audio)
            .build()
            .run(|_| ())
    }
}
