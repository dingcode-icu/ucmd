mod subcmd;

///logger about

pub use log::{debug};
///clap
pub use clap::{App, load_yaml};

use std::path::Path;
use std::collections::HashMap;
use rcmd_core::Log::init_logger;

fn main() {
    // init logger
    init_logger();
    debug!("Init logger ");
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from(yaml);
    let app_m = app.clone().get_matches();

    match app_m.subcommand() {
        Some((external, sub_m)) => {
            match external {
                "gen-conf" => {
                    subcmd::gen_conf::handle(sub_m);
                }
                "build-player" => {
                    subcmd::build_player::handle(sub_m);
                }
                "build-ab" => {
                    subcmd::build_ab::handle(sub_m);
                }
                _ => {
                    app.print_help().unwrap();
                }
            }
        }
        _ => {
            app.print_help().unwrap();
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
    let out = util::shcmd::run_sh(&cmd.to_string(), &vec!["-l", "-a"]);
}