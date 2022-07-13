pub mod _utils;

use _utils::{assets, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use pipe_trait::Pipe;
use serde_json::{json, Value as JsonValue};
use std::process::Output;

macro_rules! text_positive {
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_path:literal => $stdout:expr $(;)?
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let audio_path = assets().join($audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::new(WORKSPACE)
                .cmd
                .with_arg("get")
                .with_arg($field)
                .with_arg(audio_path)
                .output()
                .expect("execute command");
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            assert!(status.success());
            assert!(stderr.is_empty());
            assert_eq!(u8v_to_string(&stdout), $stdout);
        }
    };
}

text_positive!(title_empty0: "title" "audio0" => "");
text_positive!(title_empty1: "title" "audio1" => "");
text_positive!(title_filled2: "title" "audio2" => "砕月\n");
text_positive!(title_filled3: "title" "audio3" => "Broken Moon\n");

text_positive!(artist_empty0: "artist" "audio0" => "");
text_positive!(artist_empty1: "artist" "audio1" => "");
text_positive!(artist_filled2: "artist" "audio2" => "ココ&さつき が てんこもり\n");
text_positive!(artist_filled3: "artist" "audio3" => "Koko & Satsuki ga Tenkomori\n");

text_positive!(album_empty0: "album" "audio0" => "");
text_positive!(album_empty1: "album" "audio1" => "");
text_positive!(album_filled2: "album" "audio2" => "幻想郷ミソギバライ\n");
text_positive!(album_filled3: "album" "audio3" => "Gensoukyou Misogibarai\n");

text_positive!(album_artist_empty0: "album-artist" "audio0" => "");
text_positive!(album_artist_empty1: "album-artist" "audio1" => "");
text_positive!(album_artist_filled2: "album-artist" "audio2" => "Astral Trip\n");
text_positive!(album_artist_filled3: "album-artist" "audio3" => "Astral Trip\n");

text_positive!(genre_empty0: "genre" "audio0" => "");
text_positive!(genre_empty1: "genre" "audio1" => "");
text_positive!(genre_filled2: "genre" "audio2" => "Anime\n");
text_positive!(genre_filled3: "genre" "audio3" => "Pop\n");

mod deserialize {
    pub use serde_json::from_str as json;
    pub use serde_yaml::from_str as yaml;
}

mod serialize {
    pub use serde_json::to_string_pretty as json;
    pub use serde_yaml::to_string as yaml;
}

macro_rules! text_format_positive {
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal --format=$format:ident $audio_path:literal => $expected:expr $(;)?
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let audio_path = assets().join($audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::new(WORKSPACE)
                .cmd
                .with_arg("get")
                .with_arg($field)
                .with_arg("--format")
                .with_arg(stringify!($format))
                .with_arg(audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));

            // basic guarantees
            assert!(status.success());
            assert!(stderr.is_empty());

            // test the structured information
            let received = stdout
                .pipe_as_ref(u8v_to_string)
                .pipe(deserialize::$format::<Option<String>>)
                .expect("deserialize value");
            let expected = $expected.map(|x: &str| x.to_string());
            assert_eq!(received, expected);

            // assert that the output text is prettified
            let received = u8v_to_string(&stdout);
            let expected = $expected
                .pipe_ref(serialize::$format)
                .expect("serialize value");
            let expected = format!("{expected}\n");
            assert_eq!(received, expected);
        }
    };
}

text_format_positive!(title_json_empty0: "title" --format=json "audio0" => None::<&str>);
text_format_positive!(title_json_empty1: "title" --format=json "audio1" => None::<&str>);
text_format_positive!(title_json_filled2: "title" --format=json "audio2" => Some("砕月"));
text_format_positive!(title_json_filled3: "title" --format=json "audio3" => Some("Broken Moon"));

text_format_positive!(artist_yaml_empty0: "artist" --format=yaml "audio0" => None::<&str>);
text_format_positive!(artist_yaml_empty1: "artist" --format=yaml "audio1" => None::<&str>);
text_format_positive!(artist_yaml_filled2: "artist" --format=yaml "audio2" => Some("ココ&さつき が てんこもり"));
text_format_positive!(artist_yaml_filled3: "artist" --format=yaml "audio3" => Some("Koko & Satsuki ga Tenkomori"));

macro_rules! comment_format_positive {
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_path:literal
        --format=$format:ident
        $(--lang=$lang:ident)?
        $(--description=$description:ident)?
        => $expected:expr $(;)?
    ) => {
        #[test]
        fn $name() {
            let expected = $expected;

            let audio_path = assets().join($audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::new(WORKSPACE)
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

comment_format_positive!(comment_json_empty0: "audio0" --format=json => json!([]));
comment_format_positive!(comment_yaml_empty0: "audio0" --format=yaml => json!([]));
comment_format_positive!(comment_json_empty1: "audio1" --format=json => json!([]));
comment_format_positive!(comment_yaml_empty1: "audio1" --format=yaml => json!([]));

comment_format_positive!(comment_json_filled2: "audio2" --format=json => json!([
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format_positive!(comment_yaml_filled2: "audio2" --format=yaml => json!([
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format_positive!(comment_json_filled3: "audio3" --format=json => json!([
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

comment_format_positive!(comment_json_jpn_filled3: "audio3" --format=json --lang=jpn => json!([
    {
        "description": "",
        "lang": "jpn",
        "text": "【東方3DPV風】砕月 (ココ&さつき が てんこもり's 作業妨害Remix)"
    },
]));

comment_format_positive!(comment_yaml_filled3: "audio3" --format=yaml => json!([
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

comment_format_positive!(comment_yaml_eng_filled3: "audio3" --format=yaml --lang=eng => json!([
    {
        "description": "",
        "lang": "eng",
        "text": "【Touhou MMD PV】Broken Moon (Koko & Satsuki ga Tenkomori's Work Obstruction Remix)"
    },
]));
