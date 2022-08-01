pub mod _utils;

use _utils::{u8v_to_string, Exe};
use command_extra::CommandExtra;
use id3_cli::utils::read_tag_from_path;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! delete_all {
    // With --no-backup
    ($(#[$attributes:meta])* $name:ident: $audio_name:literal --no-backup) => {
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
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // assert that there are no tag
            let tag = read_tag_from_path(&audio_path).expect("read tag from path");
            assert_eq!(tag.frames().next(), None);
        }
    };
}

delete_all!(delete_no_backup_empty0: "audio0" --no-backup);
delete_all!(delete_no_backup_empty1: "audio1" --no-backup);
delete_all!(delete_no_backup_filled2: "audio2" --no-backup);
delete_all!(delete_no_backup_filled3: "audio3" --no-backup);
