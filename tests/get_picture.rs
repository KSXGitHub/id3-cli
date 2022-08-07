pub mod _utils;

use _utils::{assets, deserialize, serialize, sha256_file, u8v_to_string, Exe};
use command_extra::CommandExtra;
use id3_cli::text_data::{
    picture::{self, Picture},
    picture_type::PictureType,
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::{fs::read_dir, path::Path, process::Output};

macro_rules! picture {
    (
        id: $id:expr,
        mime_type: $mime_type:literal,
        picture_type: $picture_type:ident,
        description: $description:literal,
        size: $size:expr,
    ) => {
        Picture {
            mime_type: $mime_type.to_string(),
            picture_type: PictureType::$picture_type.into(),
            description: $description.to_string(),
            size: $size,
        }
        .with_id($id)
    };
}

macro_rules! get_picture_list {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal --format=$format:ident => $expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let expected = $expected;

            let audio_path = assets().join($audio_name);
            let Output {
                status,
                stdout,
                stderr,
            } = Exe::project_root()
                .cmd
                .with_arg("get")
                .with_arg("picture")
                .with_arg("list")
                .with_arg("--format")
                .with_arg(stringify!($format))
                .with_arg(audio_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());
            assert!(stderr.is_empty());

            // test the structured information
            let received = stdout
                .pipe_as_ref(u8v_to_string)
                .pipe(deserialize::$format::<Vec<picture::WithId>>)
                .expect("deserialize value");
            assert_eq!(received, expected);

            // assert that the output text is prettified
            let received = u8v_to_string(&stdout);
            let expected = expected
                .pipe_ref(serialize::$format)
                .expect("serialize value");
            let expected = format!("{expected}\n");
            assert_eq!(received, expected);
        }
    };
}

get_picture_list!(picture_list_json_empty0: "audio0" --format=json => []);
get_picture_list!(picture_list_yaml_empty0: "audio0" --format=yaml => []);
get_picture_list!(picture_list_json_empty1: "audio1" --format=json => []);
get_picture_list!(picture_list_yaml_empty1: "audio1" --format=yaml => []);

get_picture_list!(picture_list_json_audio2: "audio2" --format=json => [
    picture! {
        id: 0,
        mime_type: "image/jpeg",
        picture_type: Illustration,
        description: "砕月.jpg",
        size: 2602071,
    }
]);

get_picture_list!(picture_list_yaml_audio2: "audio2" --format=yaml => [
    picture! {
        id: 0,
        mime_type: "image/jpeg",
        picture_type: Illustration,
        description: "砕月.jpg",
        size: 2602071,
    }
]);

get_picture_list!(picture_list_json_audio3: "audio3" --format=json => [
    picture! {
        id: 0,
        mime_type: "image/png",
        picture_type: CoverFront,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png",
        size: 1874985,
    },
    picture! {
        id: 1,
        mime_type: "image/png",
        picture_type: CoverBack,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png",
        size: 1988938,
    },
    picture! {
        id: 2,
        mime_type: "image/jpeg",
        picture_type: RecordingLocation,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg",
        size: 293084,
    },
    picture! {
        id: 3,
        mime_type: "image/jpeg",
        picture_type: LeadArtist,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg",
        size: 223038,
    },
]);

get_picture_list!(picture_list_yaml_audio3: "audio3" --format=yaml => [
    picture! {
        id: 0,
        mime_type: "image/png",
        picture_type: CoverFront,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png",
        size: 1874985,
    },
    picture! {
        id: 1,
        mime_type: "image/png",
        picture_type: CoverBack,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png",
        size: 1988938,
    },
    picture! {
        id: 2,
        mime_type: "image/jpeg",
        picture_type: RecordingLocation,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg",
        size: 293084,
    },
    picture! {
        id: 3,
        mime_type: "image/jpeg",
        picture_type: LeadArtist,
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg",
        size: 223038,
    },
]);

macro_rules! get_picture_file {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal $($picture_type:literal)? => $expected:literal
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            let image_path = wdir.join("exported-image");
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("get")
                .with_arg("picture")
                .with_arg("file")
                .with_arg(&audio_path)
                .with_arg(&image_path)
                $(
                    .with_arg($picture_type)
                )?
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());
            assert!(stdout.is_empty());
            assert!(stderr.is_empty());

            // compare hash
            assert_eq!(sha256_file(image_path), $expected);
        }
    };
}

macro_rules! picture_file_fail {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal $($picture_type:literal)? => $expected:literal
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            let image_path = wdir.join("exported-image");
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("get")
                .with_arg("picture")
                .with_arg("file")
                .with_arg(&audio_path)
                .with_arg(&image_path)
                $(
                    .with_arg($picture_type)
                )?
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(!status.success());
            assert!(stdout.is_empty());

            // compare stderr
            assert_eq!(u8v_to_string(&stderr), $expected);
        }
    };
}

macro_rules! picture_file_fail_fn {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal $(--id=$id:literal)? => $get_expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let get_expected: fn(wdir: &Path) -> String = $get_expected;
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            let image_path = wdir.join("exported-image");
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("get")
                .with_arg("picture")
                .with_arg("file")
                .with_arg(&audio_path)
                .with_arg(&image_path)
                $(
                    .with_arg("--id")
                    .with_arg($id)
                )?
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(!status.success());
            assert!(stdout.is_empty());

            // compare stderr
            assert_eq!(u8v_to_string(&stderr), get_expected(wdir.as_ref()));
        }
    };
}

picture_file_fail!(picture_file_empty0: "audio0" => "error: Picture not found\n");
picture_file_fail!(picture_file_cf_empty0: "audio0" "CoverFront" => "error: Specified picture type not found\n");

picture_file_fail!(picture_file_empty1: "audio1" => "error: Picture not found\n");
picture_file_fail!(picture_file_cb_empty1: "audio1" "CoverBack" => "error: Specified picture type not found\n");

get_picture_file!(picture_file_audio2: "audio2" => "98efb430f0e307315ee46a81bfaf4ba9cf79e5996dcd227a306e1aaaf438cda4");
get_picture_file!(picture_file_il_audio2: "audio2" "Illustration" => "98efb430f0e307315ee46a81bfaf4ba9cf79e5996dcd227a306e1aaaf438cda4");
picture_file_fail!(picture_file_cf_audio2: "audio2" "CoverFront" => "error: Specified picture type not found\n");

picture_file_fail!(picture_file_audio3: "audio3" => "error: Too many pictures to choose from\n");
get_picture_file!(picture_file_cf_audio3: "audio3" "CoverFront" => "668c0693f36c2c08f8a04fd09cf9dcf38d14a52f2d65134077939a62b363d48a");
get_picture_file!(picture_file_cb_audio3: "audio3" "CoverBack" => "deaad585bd1cdbf05011c88a10cff00e630878dbc1408c1c365c19ba8ee5e169");
get_picture_file!(picture_file_rl_audio3: "audio3" "RecordingLocation" => "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a");
get_picture_file!(picture_file_la_audio3: "audio3" "LeadArtist" => "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4");
picture_file_fail!(picture_file_bl_audio3: "audio3" "BandLogo" => "error: Specified picture type not found\n");

picture_file_fail_fn!(#[cfg(unix)] picture_file_not_exist: "not-exist" => |wdir| {
    format!("error: Failed to read {:?}: No such file or directory (os error 2)\n", wdir.join("assets").join("not-exist"))
});
picture_file_fail_fn!(#[cfg(unix)] picture_file_dir: "." => |wdir| {
    format!("error: Failed to read {:?}: Is a directory (os error 21)\n", wdir.join("assets").join("."))
});

macro_rules! picture_dir_empty {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            let image_dir_path = wdir.join("exported-images");
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("get")
                .with_arg("picture")
                .with_arg("dir")
                .with_arg(&audio_path)
                .with_arg(&image_dir_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());
            assert!(stdout.is_empty());
            assert!(stderr.is_empty());

            // test the filesystem
            assert!(!image_dir_path.exists());
        }
    };
}

macro_rules! picture_dir_filled {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal => $expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_name);
            let image_dir_path = wdir.join("exported-images");
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("get")
                .with_arg("picture")
                .with_arg("dir")
                .with_arg(&audio_path)
                .with_arg(&image_dir_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // compute received hashes
            let received: Vec<_> = read_dir(&image_dir_path)
                .expect("read directory of exported images")
                .map(|entry| entry.expect("get entry").file_name())
                .map(|file_name| {
                    (
                        file_name.to_string_lossy().to_string(),
                        image_dir_path.join(file_name).pipe(sha256_file),
                    )
                })
                .collect();
            let mut received: Vec<_> = received
                .iter()
                .map(|(file_name, file_hash)| (file_name.as_ref(), file_hash.as_str()))
                .collect();
            received.sort();
            dbg!(&received);

            // compute expected
            let expected = $expected;

            // compare hashes
            assert_eq!(received, expected);
        }
    };
}

picture_dir_empty!(picture_dir_empty0: "audio0");
picture_dir_empty!(picture_dir_empty1: "audio1");

picture_dir_filled!(picture_dir_audio2: "audio2" => [(
    "0.jpg",
    "98efb430f0e307315ee46a81bfaf4ba9cf79e5996dcd227a306e1aaaf438cda4",
)]);

picture_dir_filled!(picture_dir_audio3: "audio3" => [
    (
        "0.png",
        "668c0693f36c2c08f8a04fd09cf9dcf38d14a52f2d65134077939a62b363d48a",
    ),
    (
        "1.png",
        "deaad585bd1cdbf05011c88a10cff00e630878dbc1408c1c365c19ba8ee5e169",
    ),
    (
        "2.jpg",
        "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a",
    ),
    (
        "3.jpg",
        "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4",
    ),
]);
