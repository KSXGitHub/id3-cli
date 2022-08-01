pub mod _utils;

use _utils::{assets, deserialize, serialize, u8v_to_string, Exe};
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
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            assert!(status.success());
            assert!(stderr.is_empty());
            assert_eq!(u8v_to_string(&stdout), $stdout);
        }
    };
}

get_text!(title_empty0: "title" "audio0" => "");
get_text!(title_empty1: "title" "audio1" => "");
get_text!(title_filled2: "title" "audio2" => "砕月\n");
get_text!(title_filled3: "title" "audio3" => "Broken Moon\n");

get_text!(artist_empty0: "artist" "audio0" => "");
get_text!(artist_empty1: "artist" "audio1" => "");
get_text!(artist_filled2: "artist" "audio2" => "ココ&さつき が てんこもり\n");
get_text!(artist_filled3: "artist" "audio3" => "Koko & Satsuki ga Tenkomori\n");

get_text!(album_empty0: "album" "audio0" => "");
get_text!(album_empty1: "album" "audio1" => "");
get_text!(album_filled2: "album" "audio2" => "幻想郷ミソギバライ\n");
get_text!(album_filled3: "album" "audio3" => "Gensoukyou Misogibarai\n");

get_text!(album_artist_empty0: "album-artist" "audio0" => "");
get_text!(album_artist_empty1: "album-artist" "audio1" => "");
get_text!(album_artist_filled2: "album-artist" "audio2" => "Astral Trip\n");
get_text!(album_artist_filled3: "album-artist" "audio3" => "Astral Trip\n");

get_text!(genre_name_empty0: "genre-name" "audio0" => "");
get_text!(genre_name_empty1: "genre-name" "audio1" => "");
get_text!(genre_name_filled2: "genre-name" "audio2" => "Anime\n");
get_text!(genre_name_filled3: "genre-name" "audio3" => "Pop\n");

get_text!(genre_code_empty0: "genre-code" "audio0" => "");
get_text!(genre_code_empty1: "genre-code" "audio1" => "");
get_text!(genre_code_filled2: "genre-code" "audio2" => "(145)\n");
get_text!(genre_code_filled3: "genre-code" "audio3" => "(13)\n");

get_text!(title_json_empty0: "title" --format=json "audio0" => None::<&str>);
get_text!(title_json_empty1: "title" --format=json "audio1" => None::<&str>);
get_text!(title_json_filled2: "title" --format=json "audio2" => Some("砕月"));
get_text!(title_json_filled3: "title" --format=json "audio3" => Some("Broken Moon"));

get_text!(artist_yaml_empty0: "artist" --format=yaml "audio0" => None::<&str>);
get_text!(artist_yaml_empty1: "artist" --format=yaml "audio1" => None::<&str>);
get_text!(artist_yaml_filled2: "artist" --format=yaml "audio2" => Some("ココ&さつき が てんこもり"));
get_text!(artist_yaml_filled3: "artist" --format=yaml "audio3" => Some("Koko & Satsuki ga Tenkomori"));

macro_rules! text_fail {
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
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            assert!(!status.success());
            assert!(stdout.is_empty());
            assert_eq!(u8v_to_string(&stderr), $stderr);
        }
    };
}

text_fail!(#[cfg(unix)] title_not_exist: "title" "not-exist" => format!(
    "error: Failed to read {:?}: No such file or directory (os error 2)\n",
    assets().join("not-exist"),
));
text_fail!(#[cfg(unix)] title_dir: "title" "." => format!(
    "error: Failed to read {:?}: Is a directory (os error 21)\n",
    assets().join("."),
));
