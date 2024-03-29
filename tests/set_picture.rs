pub mod _utils;

use _utils::{sha256_asset, sha256_file, show_stdout_stderr, Exe, PictureInfo, TestBackup};
use chrono::Local;
use command_extra::CommandExtra;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! set_picture {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        $picture_name:literal
        $picture_type:literal
        --no-backup
        $(--mime-type=$mime_type:literal)?
        $(--description=$description:literal)?
        => $expected_picture_list:expr
    ) => {
        $(#[$attributes])*
        #[test]
        fn $name() {
            let Exe { cmd, wdir } = Exe::temp_workspace();
            dbg!(&wdir);
            let audio_path = wdir.join("assets").join($audio_name);
            dbg!(&audio_path);
            let picture_type = $picture_type;
            dbg!(picture_type);
            let picture_path = wdir.join("assets").join($picture_name);
            dbg!(&picture_path);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg("picture")
                .with_arg("--no-backup")
                $(
                    .with_arg("--mime-type")
                    .with_arg($mime_type)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                .with_arg(&audio_path)
                .with_arg(&picture_path)
                .with_arg(picture_type)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());

            // compare picture list
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
        $name:ident:
        $audio_name:literal
        $picture_name:literal
        $picture_type:literal
        $(--mime-type=$mime_type:literal)?
        $(--description=$description:literal)?
        => $expected_picture_list:expr
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
            let picture_type = $picture_type;
            dbg!(picture_type);
            let picture_path = wdir.join("assets").join($picture_name);
            dbg!(&picture_path);
            let before_run = Local::now();
            dbg!(before_run);
            let Output {
                status,
                stdout,
                stderr,
            } = cmd
                .with_arg("set")
                .with_arg("picture")
                $(
                    .with_arg("--mime-type")
                    .with_arg($mime_type)
                )?
                $(
                    .with_arg("--description")
                    .with_arg($description)
                )?
                .with_arg(&audio_path)
                .with_arg(&picture_path)
                .with_arg(picture_type)
                .output()
                .expect("execute command");

            // for ease of debug
            show_stdout_stderr(&stdout, &stderr);

            // basic guarantees
            assert!(status.success());

            // compare picture list
            let received = PictureInfo::from_audio_file(&audio_path);
            dbg!(&received);
            let expected = $expected_picture_list;
            assert_eq!(&received, &expected);

            // make sure that a backup was created
            TestBackup::builder()
                .workspace(&wdir)
                .audio_name($audio_name)
                .initial_hash(&initial_hash)
                .before_run(before_run)
                .build()
                .test();
        }
    };
}

set_picture!(picture_no_backup_audio0_cf_jpg: "audio0" "jpg-picture" "CoverFront" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_no_backup_audio0_cb_png: "audio0" "png-picture" "CoverBack" --no-backup => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Back cover".to_string(),
        description: "png-picture".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

set_picture!(picture_no_backup_mime_audio0_cf_jpg: "audio0" "jpg-picture" "CoverFront" --no-backup --mime-type="MIME TYPE" => [
    PictureInfo {
        mime_type: "MIME TYPE".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_no_backup_desc_audio0_cb_png: "audio0" "png-picture" "CoverBack" --no-backup --description="DESCRIPTION" => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Back cover".to_string(),
        description: "DESCRIPTION".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

set_picture!(picture_no_backup_audio1_il_jpg: "audio1" "jpg-picture" "Illustration" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_no_backup_mime_desc_audio1_il_jpg: "audio1" "png-picture" "Illustration" --no-backup --mime-type="MIME" --description="DESC" => [
    PictureInfo {
        mime_type: "MIME".to_string(),
        picture_type: "Illustration".to_string(),
        description: "DESC".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

set_picture!(picture_no_backup_audio2_il_jpg: "audio2" "jpg-picture" "Illustration" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_no_backup_audio2_cf_jpg: "audio2" "jpg-picture" "CoverFront" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "砕月.jpg".to_string(),
        sha256: "98efb430f0e307315ee46a81bfaf4ba9cf79e5996dcd227a306e1aaaf438cda4".to_string(),
    },
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_no_backup_audio3_cf_jpg: "audio3" "jpg-picture" "CoverFront" --no-backup => [
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
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    },
]);

set_picture!(picture_no_backup_audio3_il_jpg: "audio3" "jpg-picture" "Illustration" --no-backup => [
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
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    },
]);

set_picture!(picture_audio0_cf_jpg: "audio0" "jpg-picture" "CoverFront" => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_mime_desc_audio1_il_jpg: "audio1" "png-picture" "Illustration" --mime-type="MIME" --description="DESC" => [
    PictureInfo {
        mime_type: "MIME".to_string(),
        picture_type: "Illustration".to_string(),
        description: "DESC".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

set_picture!(picture_audio2_il_jpg: "audio2" "jpg-picture" "Illustration" => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

set_picture!(picture_audio3_il_jpg: "audio3" "jpg-picture" "Illustration" => [
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
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    },
]);
