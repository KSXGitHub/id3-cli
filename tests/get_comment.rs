pub mod _utils;

use _utils::{assets, deserialize, serialize, u8v_to_string, Exe};
use command_extra::CommandExtra;
use id3_cli::text_data::comment::Comment;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! get_comment {
    // Without --format
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        => $expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let audio_path = assets().join($audio_name);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("comment")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                .with_arg(audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));

            // basic guarantees
            assert!(status.success());
            assert!(stderr.is_empty());

            // test stdout
            assert_eq!(u8v_to_string(&stdout), $expected);
        }
    };

    // With --format
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        --format=$format:ident
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        => $expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let expected = $expected.map(|comment: Comment<&str, &str, &str>| comment.to_owned_strings());

            let audio_path = assets().join($audio_name);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("comment")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                .with_arg("--format")
                .with_arg(stringify!($format))
                .with_arg(audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));

            // basic guarantees
            assert!(status.success());
            assert!(stderr.is_empty());

            // test the structured information
            let received = stdout
                .pipe_as_ref(u8v_to_string)
                .pipe(deserialize::$format::<Vec<Comment<String, String, String>>>)
                .expect("deserialize value");
            assert_eq!(received.as_slice(), expected);

            // assert that the output text is prettified
            let received = u8v_to_string(&stdout);
            let expected = serialize::$format(&expected).expect("serialize value");
            let expected = format!("{expected}\n");
            assert_eq!(received, expected);
        }
    };
}

macro_rules! comment_fail {
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        $(--language=$language:literal)?
        $(--description=$description:literal)?
        => $expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let audio_path = assets().join($audio_name);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("comment")
                $(
                    .with_arg("--language")
                    .with_arg($language)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                .with_arg(audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));

            // basic guarantees
            assert!(!status.success());
            assert!(stdout.is_empty());

            // test stdout
            assert_eq!(u8v_to_string(&stderr), $expected);
        }
    };
}

comment_fail!(comment_fail0: "audio0" => "error: Comment not found\n");
comment_fail!(comment_fail1: "audio1" => "error: Comment not found\n");
get_comment!(comment_audio2: "audio2" => "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)\n");
comment_fail!(comment_eng_fail2: "audio2" --language="eng" => "error: Comment not found\n");
comment_fail!(comment_fail3: "audio3" => "error: Too many comments to choose from\n");
get_comment!(comment_jpn_audio2: "audio2" --language="jpn" => "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)\n");
get_comment!(comment_eng_audio3: "audio3" --language="eng" => "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)\n");
get_comment!(comment_jpn_audio3: "audio3" --language="jpn" => "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)\n");

comment_fail!(#[cfg(unix)] comment_not_exist: "not-exist" => format!(
    "error: Failed to read {:?}: No such file or directory (os error 2)\n",
    assets().join("not-exist"),
));
comment_fail!(#[cfg(unix)] comment_dir: "." => format!(
    "error: Failed to read {:?}: Is a directory (os error 21)\n",
    assets().join("."),
));

get_comment!(comment_json_audio0: "audio0" --format=json => []);
get_comment!(comment_yaml_audio0: "audio0" --format=yaml => []);
get_comment!(comment_json_audio1: "audio1" --format=json => []);
get_comment!(comment_yaml_audio1: "audio1" --format=yaml => []);

get_comment!(comment_json_audio2: "audio2" --format=json => [
    Comment {
        description: "",
        language: "jpn",
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]);

get_comment!(comment_yaml_audio2: "audio2" --format=yaml => [
    Comment {
        description: "",
        language: "jpn",
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]);

get_comment!(comment_json_audio3: "audio3" --format=json => [
    Comment {
        description: "",
        language: "eng",
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
    Comment {
        description: "",
        language: "jpn",
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]);

get_comment!(comment_json_jpn_audio3: "audio3" --format=json --language="jpn" => [
    Comment {
        description: "",
        language: "jpn",
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]);

get_comment!(comment_yaml_audio3: "audio3" --format=yaml => [
    Comment {
        description: "",
        language: "eng",
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
    Comment {
        description: "",
        language: "jpn",
        content: "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]);

get_comment!(comment_yaml_eng_audio3: "audio3" --format=yaml --language="eng" => [
    Comment {
        description: "",
        language: "eng",
        content: "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
]);
