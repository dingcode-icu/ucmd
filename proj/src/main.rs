mod subcmd;
mod util;

use clap::{App, load_yaml, ArgMatches};
use log::log;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from(yaml);
    let app_m = app.clone().get_matches();

    match app_m.subcommand() {
        Some((external, sub_m)) => {
            match external {
                "build-player"=>{
                    subcmd::build_player::handle(sub_m);
                }
                _ => {
                    app.print_help().unwrap();
                }
            }


        }
        _ => {
            println!("do nothing");
        }
    }
}

#[test]
fn test_util() {
    // filesys
    let fp = "/Users/mac/data0/public_work/pinyin-unity-android/doc/handover";
    let tp = "/Users/mac/data0/public_work/ucmd/test1";
    util::filesys::copy_dir_all(Path::new(fp), Path::new(tp));

    //shcmd
    let cmd = "ls";
    let out = util::shcmd::run_sh(&cmd.to_string());
    println!("{}", out);

    //base cmd
    subcmd::build_player::run();

    //gen_android
    let mut map = HashMap::new();
    map.insert("a", "1");
    map.insert("b", "2");
    let out = util::gen_ios::gradle_gen("build", map);
    // asesrt!(out = "gradlew build -Pa 1 -Pb 2")
    println!("{}", out);
}