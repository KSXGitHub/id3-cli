use crate::{
    app::Run,
    backup,
    error::{Error, FileReadFailure},
    utils::sha256_data,
};
use clap::Args;
use std::{fs::read as read_file, path::PathBuf};

#[derive(Debug, Args)]
pub struct Backup {
    /// Path to the target audio file.
    pub target_audio: PathBuf,
}

impl Run for Backup {
    fn run(self) -> Result<(), Error> {
        let Backup { ref target_audio } = self;

        let audio_content = read_file(target_audio).map_err(|error| FileReadFailure {
            file: target_audio.to_path_buf(),
            error,
        })?;

        backup::Backup::builder()
            .source_file_path(target_audio)
            .source_file_hash(&sha256_data(&audio_content))
            .build()
            .backup()?;

        Ok(())
    }
}
