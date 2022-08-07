pub mod _utils;

use _utils::{sha256_file, u8v_to_string, Exe, PictureInfo, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! delete_picture {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: --no-backup $audio_name:literal $picture_type:literal => $expected_picture_list:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("delete")
                .with_arg("picture")
                .with_arg("--no-backup")
                .with_arg(&audio_path)
                .with_arg($picture_type)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // compare pictures
            let received = PictureInfo::from_audio_file(&audio_path);
            dbg!(&received);
            let expected = $expected_picture_list;
            assert_eq!(&received, &expected);

            // make sure that no backup was created
            assert!(wdir.join("assets").exists());
            assert!(!wdir.join("assets").join(".id3-backups").exists());
        }
    };

    // Without --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident: $audio_name:literal $picture_type:literal => $expected_picture_list:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let initial_hash = sha256_file(&audio_path);
            dbg!(&initial_hash);
            let before_run = Local::now();
            dbg!(before_run);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("delete")
                .with_arg("picture")
                .with_arg(&audio_path)
                .with_arg($picture_type)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

            // basic guarantees
            assert!(status.success());

            // compare pictures
            let received = PictureInfo::from_audio_file(&audio_path);
            dbg!(&received);
            let expected = $expected_picture_list;
            assert_eq!(&received, &expected);

            // make sure that a backup was created
            TestBackup::builder()
                .workspace(&wdir)
                .audio_name("audio3")
                .initial_hash(&initial_hash)
                .before_run(before_run)
                .build()
                .test();
        }
    };
}

delete_picture!(picture_no_backup_audio3_all: --no-backup "audio3" "all" => []);

delete_picture!(picture_no_backup_audio3_cf: --no-backup "audio3" "CoverFront" => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Back cover".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png".to_string(),
        sha256: "deaad585bd1cdbf05011c88a10cff00e630878dbc1408c1c365c19ba8ee5e169".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Recording location".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg".to_string(),
        sha256: "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Lead artist".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg".to_string(),
        sha256: "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4".to_string(),
    },
]);

delete_picture!(picture_no_backup_audio3_cb: --no-backup "audio3" "CoverBack" => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Front cover".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png".to_string(),
        sha256: "668c0693f36c2c08f8a04fd09cf9dcf38d14a52f2d65134077939a62b363d48a".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Recording location".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg".to_string(),
        sha256: "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Lead artist".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg".to_string(),
        sha256: "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4".to_string(),
    },
]);

delete_picture!(picture_no_backup_audio3_il: --no-backup "audio3" "Illustration" => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Front cover".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png".to_string(),
        sha256: "668c0693f36c2c08f8a04fd09cf9dcf38d14a52f2d65134077939a62b363d48a".to_string(),
    },
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Back cover".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png".to_string(),
        sha256: "deaad585bd1cdbf05011c88a10cff00e630878dbc1408c1c365c19ba8ee5e169".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Recording location".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg".to_string(),
        sha256: "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Lead artist".to_string(),
        description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg".to_string(),
        sha256: "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4".to_string(),
    },
]);

delete_picture!(picture_audio3_all: "audio3" "all" => []);

delete_picture!(picture_audio3_cf: "audio3" "CoverFront" => [
  PictureInfo {
      mime_type: "image/png".to_string(),
      picture_type: "Back cover".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png".to_string(),
      sha256: "deaad585bd1cdbf05011c88a10cff00e630878dbc1408c1c365c19ba8ee5e169".to_string(),
  },
  PictureInfo {
      mime_type: "image/jpeg".to_string(),
      picture_type: "Recording location".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg".to_string(),
      sha256: "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a".to_string(),
  },
  PictureInfo {
      mime_type: "image/jpeg".to_string(),
      picture_type: "Lead artist".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg".to_string(),
      sha256: "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4".to_string(),
  },
]);

delete_picture!(picture_audio3_cb: "audio3" "CoverBack" => [
  PictureInfo {
      mime_type: "image/png".to_string(),
      picture_type: "Front cover".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png".to_string(),
      sha256: "668c0693f36c2c08f8a04fd09cf9dcf38d14a52f2d65134077939a62b363d48a".to_string(),
  },
  PictureInfo {
      mime_type: "image/jpeg".to_string(),
      picture_type: "Recording location".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg".to_string(),
      sha256: "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a".to_string(),
  },
  PictureInfo {
      mime_type: "image/jpeg".to_string(),
      picture_type: "Lead artist".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg".to_string(),
      sha256: "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4".to_string(),
  },
]);

delete_picture!(picture_audio3_il: "audio3" "Illustration" => [
  PictureInfo {
      mime_type: "image/png".to_string(),
      picture_type: "Front cover".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:01.717.png".to_string(),
      sha256: "668c0693f36c2c08f8a04fd09cf9dcf38d14a52f2d65134077939a62b363d48a".to_string(),
  },
  PictureInfo {
      mime_type: "image/png".to_string(),
      picture_type: "Back cover".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:01:29.317.png".to_string(),
      sha256: "deaad585bd1cdbf05011c88a10cff00e630878dbc1408c1c365c19ba8ee5e169".to_string(),
  },
  PictureInfo {
      mime_type: "image/jpeg".to_string(),
      picture_type: "Recording location".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:00:48.917.jpg".to_string(),
      sha256: "96c87d647f1be8168d7b52198fe80345808e2cde8fac30733887abc9414c5a4a".to_string(),
  },
  PictureInfo {
      mime_type: "image/jpeg".to_string(),
      picture_type: "Lead artist".to_string(),
      description: "WE ARE JAPANESE GOBLIN [MMD].mp4-00:03:59.883.jpg".to_string(),
      sha256: "ff1b6b1c8a2fcb256b2f6ac5f8678dc4d185fe652d2815364b1f268475bbd4c4".to_string(),
  },
]);
