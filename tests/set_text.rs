pub mod _utils;

use _utils::{u8v_to_string, Exe, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use id3::{Tag, TagLike};
use id3_cli::utils::sha256_file;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! set_text {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal --no-backup $audio_name:literal $text:literal => $method:ident
    ) => {
        #[test]
        pub fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg($field)
                .with_arg("--no-backup")
                .with_arg(&audio_path)
                .with_arg($text)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());
            assert!(stdout.is_empty());

            // compare new value
            let tag = Tag::read_from_path(&audio_path).expect("read tag from audio");
            assert_eq!(tag.$method(), Some($text));

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };

    // Without --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_name:literal $text:literal => $method:ident
    ) => {
        #[test]
        pub fn $name() {
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
                .with_arg("set")
                .with_arg($field)
                .with_arg(&audio_path)
                .with_arg($text)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());
            assert!(stdout.is_empty());

            // compare new value
            let tag = Tag::read_from_path(&audio_path).expect("read tag from audio");
            assert_eq!(tag.$method(), Some($text));

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

set_text!(title_no_backup_audio0: "title" --no-backup "audio0" "New title" => title);
set_text!(artist_no_backup_audio1: "artist" --no-backup "audio1" "New artist" => artist);
set_text!(album_no_backup_audio2: "album" --no-backup "audio2" "New album" => album);
set_text!(album_artist_no_backup_audio3: "album-artist" --no-backup "audio3" "New album artist" => album_artist);

set_text!(title_audio0: "title" "audio0" "New title" => title);
set_text!(artist_audio1: "artist" "audio1" "New artist" => artist);
set_text!(album_audio2: "album" "audio2" "New album" => album);
set_text!(album_artist_audio3: "album-artist" "audio3" "New album artist" => album_artist);
