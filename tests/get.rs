pub mod _utils;

use _utils::{assets, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
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
