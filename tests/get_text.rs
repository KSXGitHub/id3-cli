pub mod _utils;

use _utils::{assets, deserialize, serialize, show_stdout_stderr, u8v_to_string, Exe};
use command_extra::CommandExtra;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! get_text {
    // With --format
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal --format=$format:ident $audio_name:literal => $expected:expr
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
                .with_arg($field)
                .with_arg("--format")
                .with_arg(stringify!($format))
                .with_arg(audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

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

    // Without --format
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_name:literal => $stdout:expr
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
                .with_arg($field)
                .with_arg(audio_path)
                .output()
                .expect("execute command");
            show_stdout_stderr(&stdout, &stderr);
            assert!(status.success());
            assert!(stderr.is_empty());
            assert_eq!(u8v_to_string(&stdout), $stdout);
        }
    };
}

get_text!(title_audio0: "title" "audio0" => "");
get_text!(title_audio1: "title" "audio1" => "");
get_text!(title_audio2: "title" "audio2" => "砕月\n");
get_text!(title_audio3: "title" "audio3" => "Broken Moon\n");

get_text!(artist_audio0: "artist" "audio0" => "");
get_text!(artist_audio1: "artist" "audio1" => "");
get_text!(artist_audio2: "artist" "audio2" => "ココ&さつき が てんこもり\n");
get_text!(artist_audio3: "artist" "audio3" => "Koko & Satsuki ga Tenkomori\n");

get_text!(album_audio0: "album" "audio0" => "");
get_text!(album_audio1: "album" "audio1" => "");
get_text!(album_audio2: "album" "audio2" => "幻想郷ミソギバライ\n");
get_text!(album_audio3: "album" "audio3" => "Gensoukyou Misogibarai\n");

get_text!(album_artist_audio0: "album-artist" "audio0" => "");
get_text!(album_artist_audio1: "album-artist" "audio1" => "");
get_text!(album_artist_audio2: "album-artist" "audio2" => "Astral Trip\n");
get_text!(album_artist_audio3: "album-artist" "audio3" => "Astral Trip\n");

get_text!(genre_name_audio0: "genre-name" "audio0" => "");
get_text!(genre_name_audio1: "genre-name" "audio1" => "");
get_text!(genre_name_audio2: "genre-name" "audio2" => "Anime\n");
get_text!(genre_name_audio3: "genre-name" "audio3" => "Pop\n");

get_text!(genre_code_audio0: "genre-code" "audio0" => "");
get_text!(genre_code_audio1: "genre-code" "audio1" => "");
get_text!(genre_code_audio2: "genre-code" "audio2" => "(145)\n");
get_text!(genre_code_audio3: "genre-code" "audio3" => "(13)\n");

get_text!(title_json_audio0: "title" --format=json "audio0" => None::<&str>);
get_text!(title_json_audio1: "title" --format=json "audio1" => None::<&str>);
get_text!(title_json_audio2: "title" --format=json "audio2" => Some("砕月"));
get_text!(title_json_audio3: "title" --format=json "audio3" => Some("Broken Moon"));

get_text!(artist_yaml_audio0: "artist" --format=yaml "audio0" => None::<&str>);
get_text!(artist_yaml_audio1: "artist" --format=yaml "audio1" => None::<&str>);
get_text!(artist_yaml_audio2: "artist" --format=yaml "audio2" => Some("ココ&さつき が てんこもり"));
get_text!(artist_yaml_audio3: "artist" --format=yaml "audio3" => Some("Koko & Satsuki ga Tenkomori"));

macro_rules! get_text_fail {
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_name:literal => $stderr:expr
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
                .with_arg($field)
                .with_arg(audio_path)
                .output()
                .expect("execute command");
            show_stdout_stderr(&stdout, &stderr);
            assert!(!status.success());
            assert!(stdout.is_empty());
            assert_eq!(u8v_to_string(&stderr), $stderr);
        }
    };
}

get_text_fail!(#[cfg(unix)] title_not_exist: "title" "not-exist" => format!(
    "error: Failed to read {:?}: No such file or directory (os error 2)\n",
    assets().join("not-exist"),
));
get_text_fail!(#[cfg(unix)] title_dir: "title" "." => format!(
    "error: Failed to read {:?}: Is a directory (os error 21)\n",
    assets().join("."),
));
