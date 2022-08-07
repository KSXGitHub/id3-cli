pub mod _utils;

use _utils::{sha256_file, show_stdout_stderr, Exe, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use id3::{Tag, TagLike};
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! delete_text {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal --no-backup $audio_name:literal => $method:ident
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("delete")
                .with_arg($field)
                .with_arg("--no-backup")
                .with_arg(&audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());
            assert!(stdout.is_empty());

            // assert that the text field is deleted.
            let tag = Tag::read_from_path(&audio_path).expect("read tag from audio");
            assert_eq!(tag.$method(), None);

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };

    // Without --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_name:literal => $method:ident
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
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
                .with_arg($field)
                .with_arg(&audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());
            assert!(stdout.is_empty());

            // assert that the text field is deleted.
            let tag = Tag::read_from_path(&audio_path).expect("read tag from audio");
            assert_eq!(tag.$method(), None);

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

delete_text!(title_no_backup_audio0: "title" --no-backup "audio0" => title);
delete_text!(artist_no_backup_audio1: "artist" --no-backup "audio1" => artist);
delete_text!(album_no_backup_audio2: "album" --no-backup "audio2" => album);
delete_text!(genre_no_backup_audio3: "genre" --no-backup "audio3" => genre);

delete_text!(title_audio0: "title" "audio0" => title);
delete_text!(artist_audio1: "artist" "audio1" => artist);
delete_text!(album_audio2: "album" "audio2" => album);
delete_text!(genre_audio3: "genre" "audio3" => genre);
