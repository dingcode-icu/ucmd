use std::process::Command;
use std::borrow::{Cow, Borrow};

pub fn run_sh(test : &String) -> String {
    let child = Command::new(test)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute cmd {0}", e) );
    println!("output：");
    let out = String::from_utf8_lossy(&child.stdout).into_owned();
    return out;
}