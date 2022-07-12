pub mod _utils;

use _utils::{audio0, audio1, audio2, audio3, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use std::process::Output;

macro_rules! text_positive {
    (
        $(#[$attributes:meta])*
        $name:ident: $field:literal $audio_path:expr => $stdout:expr $(;)?
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::new(WORKSPACE)
                .cmd
                .with_arg("get")
                .with_arg($field)
                .with_arg($audio_path)
                .output()
                .expect("execute command");
            assert!(status.success());
            assert!(stderr.is_empty());
            assert_eq!(u8v_to_string(&stdout), $stdout);
        }
    };
}

text_positive!(title_empty0: "title" audio0() => "");
text_positive!(title_empty1: "title" audio1() => "");
text_positive!(title_positive2: "title" audio2() => "砕月\n");
text_positive!(title_positive3: "title" audio3() => "Broken Moon\n");
