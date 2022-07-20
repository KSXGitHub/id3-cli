pub mod _utils;

use _utils::{serialize, u8v_to_string, Exe, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use id3::Tag;
use id3_cli::{text_data::comment::Comment, utils::sha256_file};
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! infer {
    (language) => {
        "\0\0\0"
    };
    (language $language:literal) => {
        $language
    };
    (description) => {
        ""
    };
    (description $description:literal) => {
        $description
    };
}

macro_rules! format_comment {
    ($comment:ident) => {
        $comment
    };
    ($comment:ident as $format:ident) => {
        serialize::$format(&$comment).expect("serialize comment")
    };
}

macro_rules! comment {
    // With --no-backup, without ejection
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        --no-backup
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        $(--format=$format:ident)?
    ) => {
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let content = "This is a new comment";
            let expected_injected_comment = Comment {
                language: infer!(language $($language)?),
                description: infer!(description $($description)?),
                content,
            };
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg("comment")
                .with_arg("--no-backup")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                $(
                    .with_arg("--format")
                    .with_arg(stringify!($format))
                )?
                .with_arg(&audio_path)
                .with_arg(content)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // assert that comment was injected
            let comment_injected = Tag::read_from_path(&audio_path)
                .expect("read tag from audio")
                .comments()
                .map(Comment::from)
                .any(|comment| comment == expected_injected_comment);
            assert!(comment_injected);

            // assert that there's no ejected comment
            assert!(stdout.is_empty());

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };

    // With --no-backup, with ejection
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        --no-backup
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        $(--format=$format:ident)?
        => $ejected:expr
    ) => {
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let content = "This is a new comment";
            let expected_injected_comment = Comment {
                language: infer!(language $($language)?),
                description: infer!(description $($description)?),
                content,
            };
            let expected_ejected_comment = $ejected;
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg("comment")
                .with_arg("--no-backup")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                $(
                    .with_arg("--format")
                    .with_arg(stringify!($format))
                )?
                .with_arg(&audio_path)
                .with_arg(content)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // assert that comment was injected
            let comment_injected = Tag::read_from_path(&audio_path)
                .expect("read tag from audio")
                .comments()
                .map(Comment::from)
                .any(|comment| comment == expected_injected_comment);
            assert!(comment_injected);

            // compare ejected comment
            let received = u8v_to_string(&stdout);
            let expected = format!("{}\n", format_comment!(expected_ejected_comment $(as $format)?));
            assert_eq!(received, expected);

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };

    // Without --no-backup, without ejection
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        $(--format=$format:ident)?
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
            let content = "This is a new comment";
            let expected_injected_comment = Comment {
                language: infer!(language $($language)?),
                description: infer!(description $($description)?),
                content,
            };
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg("comment")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                $(
                    .with_arg("--format")
                    .with_arg(stringify!($format))
                )?
                .with_arg(&audio_path)
                .with_arg(content)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // assert that comment was injected
            let comment_injected = Tag::read_from_path(&audio_path)
                .expect("read tag from audio")
                .comments()
                .map(Comment::from)
                .any(|comment| comment == expected_injected_comment);
            assert!(comment_injected);

            // assert that there's no ejected comment
            assert!(stdout.is_empty());

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

    // With --no-backup, with ejection
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        $(--format=$format:ident)?
        => $ejected:expr
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
            let content = "This is a new comment";
            let expected_injected_comment = Comment {
                language: infer!(language $($language)?),
                description: infer!(description $($description)?),
                content,
            };
            let expected_ejected_comment = $ejected;
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg("comment")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                $(
                    .with_arg("--format")
                    .with_arg(stringify!($format))
                )?
                .with_arg(&audio_path)
                .with_arg(content)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // assert that comment was injected
            let comment_injected = Tag::read_from_path(&audio_path)
                .expect("read tag from audio")
                .comments()
                .map(Comment::from)
                .any(|comment| comment == expected_injected_comment);
            assert!(comment_injected);

            // compare ejected comment
            let received = u8v_to_string(&stdout);
            let expected = format!("{}\n", format_comment!(expected_ejected_comment $(as $format)?));
            assert_eq!(received, expected);

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

comment!(comment_no_backup_empty0: "audio0" --no-backup);
comment!(comment_no_backup_empty1: "audio1" --no-backup);

comment!(comment_no_backup_filled2: "audio2" --no-backup);
comment!(comment_no_backup_eng_filled2: "audio2" --no-backup --language="eng");
comment!(comment_no_backup_jpn_filled2: "audio2" --no-backup --language="jpn" => {
    "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
});

comment!(comment_no_backup_eng_filled3: "audio3" --no-backup --language="eng" => {
    "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
});
comment!(comment_no_backup_eng_json_filled3: "audio3" --no-backup --language="eng" --format=json => Comment {
    language: "eng",
    description: "",
    content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)",
});
comment!(comment_no_backup_eng_yaml_filled3: "audio3" --no-backup --language="eng" --format=yaml => Comment {
    language: "eng",
    description: "",
    content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)",
});
comment!(comment_no_backup_jpn_filled3: "audio3" --no-backup --language="jpn" => {
    "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
});
comment!(comment_no_backup_jpn_json_filled3: "audio3" --no-backup --language="jpn" --format=json => Comment {
    language: "jpn",
    description: "",
    content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)",
});
comment!(comment_no_backup_jpn_yaml_filled3: "audio3" --no-backup --language="jpn" --format=yaml => Comment {
    language: "jpn",
    description: "",
    content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)",
});

comment!(comment_empty0: "audio0");
comment!(comment_empty1: "audio1");

comment!(comment_filled2: "audio2");
comment!(comment_eng_filled2: "audio2" --language="eng");
comment!(comment_jpn_filled2: "audio2" --language="jpn" => {
    "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
});

comment!(comment_eng_filled3: "audio3" --language="eng" => {
    "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
});
comment!(comment_eng_json_filled3: "audio3" --language="eng" --format=json => Comment {
    language: "eng",
    description: "",
    content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)",
});
comment!(comment_eng_yaml_filled3: "audio3" --language="eng" --format=yaml => Comment {
    language: "eng",
    description: "",
    content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)",
});
comment!(comment_jpn_filled3: "audio3" --language="jpn" => {
    "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
});
comment!(comment_jpn_json_filled3: "audio3" --language="jpn" --format=json => Comment {
    language: "jpn",
    description: "",
    content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)",
});
comment!(comment_jpn_yaml_filled3: "audio3" --language="jpn" --format=yaml => Comment {
    language: "jpn",
    description: "",
    content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)",
});
