pub mod _utils;

use _utils::{u8v_to_string, Exe};
use command_extra::CommandExtra;
use id3::{Tag, TagLike};
use pretty_assertions::assert_eq;
use std::process::Output;

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
}

set_text!(title_no_backup_audio0: "title" --no-backup "audio0" "New title" => title);
set_text!(artist_no_backup_audio1: "artist" --no-backup "audio1" "New artist" => artist);
set_text!(album_no_backup_audio2: "album" --no-backup "audio2" "New album" => album);
set_text!(album_artist_no_backup_audio3: "album-artist" --no-backup "audio3" "New album artist" => album_artist);
