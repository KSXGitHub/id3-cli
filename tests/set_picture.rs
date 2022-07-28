pub mod _utils;

use _utils::{sha256_asset, u8v_to_string, Exe, PictureInfo};
use command_extra::CommandExtra;
use pretty_assertions::assert_eq;
use std::process::Output;

macro_rules! picture {
    // With --no-backup
    (
        $(#[$attributes:meta])*
        $name:ident:
        $audio_name:literal
        $picture_type:literal
        $picture_name:literal
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
            let picture_path = wdir.join("assets").join($picture_name);
            dbg!(&picture_path);
            let picture_type = $picture_type;
            dbg!(picture_type);
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
                .with_arg(picture_type)
                .with_arg(&picture_path)
                .output()
                .expect("execute command");

            // for ease of debug
            eprintln!("STDERR:\n{}", u8v_to_string(&stderr));
            eprintln!("STDOUT:\n{}", u8v_to_string(&stdout));

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
}

picture!(picture_no_backup_empty0_cf_jpg: "audio0" "CoverFront" "jpg-picture" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

picture!(picture_no_backup_empty0_cb_png: "audio0" "CoverBack" "png-picture" --no-backup => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Back cover".to_string(),
        description: "png-picture".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

picture!(picture_no_backup_mime_empty0_cf_jpg: "audio0" "CoverFront" "jpg-picture" --no-backup --mime-type="MIME TYPE" => [
    PictureInfo {
        mime_type: "MIME TYPE".to_string(),
        picture_type: "Front cover".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

picture!(picture_no_backup_desc_empty0_cb_png: "audio0" "CoverBack" "png-picture" --no-backup --description="DESCRIPTION" => [
    PictureInfo {
        mime_type: "image/png".to_string(),
        picture_type: "Back cover".to_string(),
        description: "DESCRIPTION".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

picture!(picture_no_backup_empty1_il_jpg: "audio1" "Illustration" "jpg-picture" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

picture!(picture_no_backup_mime_desc_empty1_il_jpg: "audio1" "Illustration" "png-picture" --no-backup --mime-type="MIME" --description="DESC" => [
    PictureInfo {
        mime_type: "MIME".to_string(),
        picture_type: "Illustration".to_string(),
        description: "DESC".to_string(),
        sha256: sha256_asset("png-picture"),
    }
]);

picture!(picture_no_backup_filled2_il_jpg: "audio2" "Illustration" "jpg-picture" --no-backup => [
    PictureInfo {
        mime_type: "image/jpeg".to_string(),
        picture_type: "Illustration".to_string(),
        description: "jpg-picture".to_string(),
        sha256: sha256_asset("jpg-picture"),
    }
]);

picture!(picture_no_backup_filled2_cf_jpg: "audio2" "CoverFront" "jpg-picture" --no-backup => [
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

picture!(picture_no_backup_filled3_cf_jpg: "audio3" "CoverFront" "jpg-picture" --no-backup => [
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

picture!(picture_no_backup_filled3_il_jpg: "audio3" "Illustration" "jpg-picture" --no-backup => [
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
