pub mod _utils;

use _utils::{assets, deserialize, serialize, u8v_to_string, Exe};
use command_extra::CommandExtra;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use serde_json::{json, Value as JsonValue};
use std::process::Output;

macro_rules! comment {
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_path:literal
        $(--lang=$lang:ident)?
        $(--description=$description:ident)?
        => $expected:expr
    ) => {
        #[test]
        fn $name() {
            let audio_path = assets().join($audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("comment")
                $(
                    .with_arg("--lang")
                    .with_arg(stringify!($lang))
                )?
                $(
                    .with_arg("--description")
                    .with_arg(stringify!($description))
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
}

macro_rules! comment_fail {
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_path:literal
        $(--lang=$lang:ident)?
        $(--description=$description:ident)?
        => $expected:expr
    ) => {
        #[test]
        fn $name() {
            let audio_path = assets().join($audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("comment")
                $(
                    .with_arg("--lang")
                    .with_arg(stringify!($lang))
                )?
                $(
                    .with_arg("--description")
                    .with_arg(stringify!($description))
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
comment!(comment_filled2: "audio2" => "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)\n");
comment_fail!(comment_eng_fail2: "audio2" --lang=eng => "error: Comment not found\n");
comment_fail!(comment_fail3: "audio3" => "error: Too many comments to choose from\n");
comment!(comment_jpn_filled2: "audio2" --lang=jpn => "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)\n");
comment!(comment_eng_filled3: "audio3" --lang=eng => "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)\n");
comment!(comment_jpn_filled3: "audio3" --lang=jpn => "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)\n");

comment_fail!(#[cfg(unix)] comment_not_exist: "not-exist" => format!(
    "error: Failed to read {:?}: No such file or directory (os error 2)\n",
    assets().join("not-exist"),
));
comment_fail!(#[cfg(unix)] comment_dir: "." => format!(
    "error: Failed to read {:?}: Is a directory (os error 21)\n",
    assets().join("."),
));

macro_rules! comment_format {
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_path:literal
        --format=$format:ident
        $(--lang=$lang:ident)?
        $(--description=$description:ident)?
        => $expected:expr
    ) => {
        #[test]
        fn $name() {
            let expected = $expected;

            let audio_path = assets().join($audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("comment")
                $(
                    .with_arg("--lang")
                    .with_arg(stringify!($lang))
                )?
                $(
                    .with_arg("--description")
                    .with_arg(stringify!($description))
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
                .pipe(deserialize::$format::<JsonValue>)
                .expect("deserialize value");
            assert_eq!(received, expected);

            // assert that the output text is prettified
            let received = u8v_to_string(&stdout);
            let expected = serialize::$format(&expected).expect("serialize value");
            let expected = format!("{expected}\n");
            assert_eq!(received, expected);
        }
    };
}

comment_format!(comment_json_empty0: "audio0" --format=json => json!([]));
comment_format!(comment_yaml_empty0: "audio0" --format=yaml => json!([]));
comment_format!(comment_json_empty1: "audio1" --format=json => json!([]));
comment_format!(comment_yaml_empty1: "audio1" --format=yaml => json!([]));

comment_format!(comment_json_filled2: "audio2" --format=json => json!([
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format!(comment_yaml_filled2: "audio2" --format=yaml => json!([
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format!(comment_json_filled3: "audio3" --format=json => json!([
    {
        "description": "",
        "lang": "eng",
        "text": "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format!(comment_json_jpn_filled3: "audio3" --format=json --lang=jpn => json!([
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format!(comment_yaml_filled3: "audio3" --format=yaml => json!([
    {
        "description": "",
        "lang": "eng",
        "text": "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format!(comment_yaml_eng_filled3: "audio3" --format=yaml --lang=eng => json!([
    {
        "description": "",
        "lang": "eng",
        "text": "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
]));
