pub mod _utils;

use _utils::{sha256_file, show_stdout_stderr, Exe, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use std::process::Output;

macro_rules! backup {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal
    ) => {
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let initial_hash = sha256_file(&audio_path);
            dbg!(&initial_hash);
            let before_run = Local::now();
            dbg!(before_run);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("backup")
                .with_arg(&audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());

            // make sure that a backup was created
            TestBackup::builder()
                .workspace(&wdir)
                .audio_name($audio_name)
                .initial_hash(&initial_hash)
                .before_run(before_run)
                .build()
                .test();
        }
    };
}

backup!(backup_audio0: "audio0");
backup!(backup_audio1: "audio1");
backup!(backup_audio2: "audio2");
backup!(backup_audio3: "audio3");
