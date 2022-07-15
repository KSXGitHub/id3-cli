pub mod _utils;

use _utils::{assets, deserialize, serialize, sha256_file, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use id3_cli::text_data::picture::{self, Picture};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::{fs::read_dir, process::Output};

macro_rules! picture {
    (
        id: $id:expr,
        mime_type: $mime_type:literal,
        picture_type: $picture_type:literal,
        description: $description:literal,
        size: $size:expr,
    ) => {
        Picture {
            mime_type: $mime_type.to_string(),
            picture_type: $picture_type.to_string(),
            description: $description.to_string(),
            size: $size,
        }
        .with_id($id)
    };
}

macro_rules! picture_list {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_path:literal --format=$format:ident => $expected:expr
    ) => {
        $(#[$attributes])*
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

picture_list!(picture_list_json_empty0: "audio0" --format=json => []);
picture_list!(picture_list_yaml_empty0: "audio0" --format=yaml => []);
picture_list!(picture_list_json_empty1: "audio1" --format=json => []);
picture_list!(picture_list_yaml_empty1: "audio1" --format=yaml => []);

picture_list!(picture_list_json_filled2: "audio2" --format=json => [
    picture! {
        id: 0,
        mime_type: "image/jpeg",
        picture_type: "Illustration",
        description: "砕月.jpg",
        size: 2602071,
    }
]);

picture_list!(picture_list_yaml_filled2: "audio2" --format=yaml => [
    picture! {
        id: 0,
        mime_type: "image/jpeg",
        picture_type: "Illustration",
        description: "砕月.jpg",
        size: 2602071,
    }
]);

picture_list!(picture_list_json_filled3: "audio3" --format=json => [
    picture! {
        id: 0,
        mime_type: "image/png",
        picture_type: "Front cover",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png",
        size: 1874985,
    },
    picture! {
        id: 1,
        mime_type: "image/png",
        picture_type: "Back cover",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png",
        size: 1988938,
    },
    picture! {
        id: 2,
        mime_type: "image/jpeg",
        picture_type: "Recording location",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg",
        size: 293084,
    },
    picture! {
        id: 3,
        mime_type: "image/jpeg",
        picture_type: "Lead artist",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg",
        size: 223038,
    },
]);

picture_list!(picture_list_yaml_filled3: "audio3" --format=yaml => [
    picture! {
        id: 0,
        mime_type: "image/png",
        picture_type: "Front cover",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png",
        size: 1874985,
    },
    picture! {
        id: 1,
        mime_type: "image/png",
        picture_type: "Back cover",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png",
        size: 1988938,
    },
    picture! {
        id: 2,
        mime_type: "image/jpeg",
        picture_type: "Recording location",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg",
        size: 293084,
    },
    picture! {
        id: 3,
        mime_type: "image/jpeg",
        picture_type: "Lead artist",
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg",
        size: 223038,
    },
]);

macro_rules! picture_dir_empty {
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_path:literal
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_path);
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
        $name:ident: $audio_path:literal => $expected:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            let audio_path = wdir.join("assets").join($audio_path);
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

picture_dir_filled!(picture_dir_filled2: "audio2" => [(
    "0.jpg",
    "98efb430f0e307315ee46a81bfaf4ba9cf79e5996dcd227a306e1aaaf438cda4",
)]);

picture_dir_filled!(picture_dir_filled3: "audio3" => [
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
