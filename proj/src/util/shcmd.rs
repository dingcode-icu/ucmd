use std::process::{Command, Stdio, ExitStatus};
use std::borrow::{Cow, Borrow};
use std::io::Read;

pub fn run_sh(cmd: &String, arg: &Vec<&str>) ->(bool, String) {
    println!("run cmd bin:{}\n", cmd);
    println!("args is :{:?}\n", arg);
    let child = Command::new(cmd)
        .args(arg)
        .output()
        .expect("failed to execute child");
    let ret = String::from_utf8_lossy(&child.stdout).into_owned();
    if child.status.success(){
        return (true, ret)
    }
    return (false, String::from_utf8_lossy(&child.stdout).into_owned());
}

pub fn run_sh_async(cmd: &String, arg: &Vec<&str>) ->(bool, String) {
    println!("run cmd bin:{}\n", cmd);
    println!("args is :{:?}\n", arg);
    let child = Command::new(cmd)
        .args(arg)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");
    let ret = String::from_utf8_lossy(&output.stdout).into_owned();
    if output.status.success(){
        return (true, ret)
    }
    return (false, String::from_utf8_lossy(&output.stderr).into_owned());
}