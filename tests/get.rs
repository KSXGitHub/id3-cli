pub mod _utils;

use _utils::{audio3, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use std::process::Output;

#[test]
fn title_positive3() {
    let Output {
        status,
        stdout,
        stderr,
    } = Exe::new(WORKSPACE)
        .cmd
        .with_arg("get")
        .with_arg("title")
        .with_arg(audio3())
        .output()
        .expect("execute command");
    assert!(status.success());
    assert!(stderr.is_empty());
    assert_eq!(u8v_to_string(&stdout), "Broken Moon\n");
}
