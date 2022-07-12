pub mod _utils;

use _utils::{audio_path, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use std::process::Output;

#[test]
fn title() {
    let Output {
        status,
        stdout,
        stderr,
    } = Exe::new(WORKSPACE)
        .cmd
        .with_arg("get")
        .with_arg("title")
        .with_arg(audio_path())
        .output()
        .expect("execute command");
    assert!(status.success());
    assert!(stderr.is_empty());
    assert_eq!(u8v_to_string(&stdout), "Broken Moon\n");
}
