pub mod _utils;

use _utils::{assets, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use pipe_trait::Pipe;
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
text_positive!(title_positive2: "title" "audio2" => "砕月\n");
text_positive!(title_positive3: "title" "audio3" => "Broken Moon\n");

text_positive!(artist_empty0: "artist" "audio0" => "");
text_positive!(artist_empty1: "artist" "audio1" => "");
text_positive!(artist_positive2: "artist" "audio2" => "ココ&さつき が てんこもり\n");
text_positive!(artist_positive3: "artist" "audio3" => "Koko & Satsuki ga Tenkomori\n");

text_positive!(album_empty0: "album" "audio0" => "");
text_positive!(album_empty1: "album" "audio1" => "");
text_positive!(album_positive2: "album" "audio2" => "幻想郷ミソギバライ\n");
text_positive!(album_positive3: "album" "audio3" => "Gensoukyou Misogibarai\n");

text_positive!(album_artist_empty0: "album-artist" "audio0" => "");
text_positive!(album_artist_empty1: "album-artist" "audio1" => "");
text_positive!(album_artist_positive2: "album-artist" "audio2" => "Astral Trip\n");
text_positive!(album_artist_positive3: "album-artist" "audio3" => "Astral Trip\n");

text_positive!(genre_empty0: "genre" "audio0" => "");
text_positive!(genre_empty1: "genre" "audio1" => "");
text_positive!(genre_positive2: "genre" "audio2" => "Anime\n");
text_positive!(genre_positive3: "genre" "audio3" => "Pop\n");

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
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            assert!(status.success());
            assert!(stderr.is_empty());
            let received = stdout
                .pipe_as_ref(u8v_to_string)
                .pipe(deserialize::$format::<Option<String>>)
                .expect("deserialize value");
            let expected = $expected.map(|x: &str| x.to_string());
            assert_eq!(received, expected);
        }
    };
}

mod deserialize {
    pub use serde_json::from_str as json;
    pub use serde_yaml::from_str as yaml;
    pub use toml::de::from_str as toml;
}

text_format_positive!(title_json_empty0: "title" --format=json "audio0" => None);
text_format_positive!(title_json_empty1: "title" --format=json "audio1" => None);
text_format_positive!(title_json_positive2: "title" --format=json "audio2" => Some("砕月"));
text_format_positive!(title_json_positive3: "title" --format=json "audio3" => Some("Broken Moon"));

text_format_positive!(artist_yaml_empty0: "artist" --format=yaml "audio0" => None);
text_format_positive!(artist_yaml_empty1: "artist" --format=yaml "audio1" => None);
text_format_positive!(artist_yaml_positive2: "artist" --format=yaml "audio2" => Some("ココ&さつき が てんこもり"));
text_format_positive!(artist_yaml_positive3: "artist" --format=yaml "audio3" => Some("Koko & Satsuki ga Tenkomori"));
