pub mod _utils;

use _utils::{sha256_file, show_stdout_stderr, Exe, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use id3_cli::utils::read_tag_from_path;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! delete_all {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal --no-backup
    ) => {
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("delete")
                .with_arg("all")
                .with_arg("--no-backup")
                .with_arg(&audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());

            // assert that there are no tag
            let tag = read_tag_from_path(&audio_path).expect("read tag from path");
            assert_eq!(tag.frames().next(), None);

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };

    // Without --no-backup
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
                .with_arg("delete")
                .with_arg("all")
                .with_arg(&audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());

            // assert that there are no tag
            let tag = read_tag_from_path(&audio_path).expect("read tag from path");
            assert_eq!(tag.frames().next(), None);

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

delete_all!(delete_no_backup_audio0: "audio0" --no-backup);
delete_all!(delete_no_backup_audio1: "audio1" --no-backup);
delete_all!(delete_no_backup_audio2: "audio2" --no-backup);
delete_all!(delete_no_backup_audio3: "audio3" --no-backup);

delete_all!(delete_audio0: "audio0");
delete_all!(delete_audio1: "audio1");
delete_all!(delete_audio2: "audio2");
delete_all!(delete_audio3: "audio3");
