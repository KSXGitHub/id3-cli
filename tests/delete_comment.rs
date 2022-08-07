pub mod _utils;

use _utils::{sha256_file, show_stdout_stderr, CommentInfo, Exe, TestBackup};
use chrono::Local;
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
            show_stdout_stderr(&stdout, &stderr);

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

    // Without --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
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
                .with_arg("comment")
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
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());

            // compare comments
            let received = CommentInfo::from_audio_file(&audio_path);
            dbg!(&received);
            let expected = $expected_comment_list;
            assert_eq!(&received, &expected);

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

delete_comment!(comment_audio3: "audio3"=> []);

delete_comment!(comment_content_jpn_audio3: "audio3"--content="【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
]);

delete_comment!(comment_desc_empty_audio3: "audio3"--description="" => []);

delete_comment!(comment_desc_empty_content_jpn_audio3: "audio3"--description="" --content="【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)" => [
    CommentInfo {
        language: "eng".to_string(),
        description: "".to_string(),
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)".to_string(),
    },
]);

delete_comment!(comment_desc_empty_content_eng_audio3: "audio3"--description="" --content="【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)" => [
    CommentInfo {
        language: "jpn".to_string(),
        description: "".to_string(),
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)".to_string(),
    },
]);

delete_comment!(comment_desc_unmatched_content_eng_audio3: "audio3"--description="description" --content="【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)" => [
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

delete_comment!(comment_desc_empty_content_unmatched_audio3: "audio3"--description="" --content="content" => [
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

delete_comment!(comment_desc_unmatched_content_unmatched_audio3: "audio3"--description="description" --content="content" => [
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
