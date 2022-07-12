pub mod _utils;

use _utils::{audio1, audio2, audio3, u8v_to_string, Exe, WORKSPACE};
use command_extra::CommandExtra;
use std::process::Output;

#[test]
fn title_empty1() {
    let Output {
        status,
        stdout,
        stderr,
    } = Exe::new(WORKSPACE)
        .cmd
        .with_arg("get")
        .with_arg("title")
        .with_arg(audio1())
        .output()
        .expect("execute command");
    assert!(status.success());
    assert!(stderr.is_empty());
    assert_eq!(u8v_to_string(&stdout), "");
}

#[test]
fn title_positive2() {
    let Output {
        status,
        stdout,
        stderr,
    } = Exe::new(WORKSPACE)
        .cmd
        .with_arg("get")
        .with_arg("title")
        .with_arg(audio2())
        .output()
        .expect("execute command");
    assert!(status.success());
    assert!(stderr.is_empty());
    assert_eq!(u8v_to_string(&stdout), "砕月\n");
}

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
