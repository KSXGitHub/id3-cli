pub mod _utils;

use _utils::{u8v_to_string, CommentInfo, Exe};
use command_extra::CommandExtra;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! delete_comment {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        --no-backup
        $(--description=$description:literal)?
        $(--content=$content:literal)?
        => $expected_comment_list:expr
    ) => {
        $(#[$attributes])*
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
                .with_arg("comment")
                .with_arg("--no-backup")
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                $(
                    .with_arg("--content")
                    .with_arg($content)
                )?
                .with_arg(&audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // compare comments
            let received = CommentInfo::from_audio_file(&audio_path);
            dbg!(&received);
            let expected = $expected_comment_list;
            assert_eq!(&received, &expected);

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };
}

delete_comment!(comment_no_backup_audio3: "audio3" --no-backup => []);

delete_comment!(comment_no_backup_content_jpn_audio3: "audio3" --no-backup --content="【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
]);

delete_comment!(comment_no_backup_desc_empty_audio3: "audio3" --no-backup --description="" => []);

delete_comment!(comment_no_backup_desc_empty_content_jpn_audio3: "audio3" --no-backup --description="" --content="【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
]);

delete_comment!(comment_no_backup_desc_empty_content_eng_audio3: "audio3" --no-backup --description="" --content="【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)" => [
    CommentInfo {
        language: "jpn".to_string(),
        description: "".to_string(),
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)".to_string(),
    },
]);

delete_comment!(comment_no_backup_desc_unmatched_content_eng_audio3: "audio3" --no-backup --description="description" --content="【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
    CommentInfo {
        language: "jpn".to_string(),
        description: "".to_string(),
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)".to_string(),
    },
]);

delete_comment!(comment_no_backup_desc_empty_content_unmatched_audio3: "audio3" --no-backup --description="" --content="content" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
    CommentInfo {
        language: "jpn".to_string(),
        description: "".to_string(),
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)".to_string(),
    },
]);

delete_comment!(comment_no_backup_desc_unmatched_content_unmatched_audio3: "audio3" --no-backup --description="description" --content="content" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
    CommentInfo {
        language: "jpn".to_string(),
        description: "".to_string(),
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)".to_string(),
    },
]);
