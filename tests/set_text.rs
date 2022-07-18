pub mod _utils;

use _utils::{u8v_to_string, Exe};
use command_extra::CommandExtra;
use id3::{Tag, TagLike};
use id3_cli::utils::sha256_file;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::{fs::read_dir, process::Output};

macro_rules! set_text {
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal --no-backup $audio_path:literal $text:literal => $method:ident
    ) => {
        #[test]
        pub fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_path);
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

    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_path:literal $text:literal => $method:ident
    ) => {
        #[test]
        pub fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_path);
            dbg!(&audio_path);
            let initial_hash = sha256_file(&audio_path);
            dbg!(&initial_hash);
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
            let backup_path: Vec<_> = wdir
                .join("assets")
                .join(".id3-backups")
                .join($audio_path)
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
            assert_eq!(sha256_file(backup_path), initial_hash);
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
