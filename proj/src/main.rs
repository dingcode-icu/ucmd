mod subcmd;
mod util;

use clap::{App, load_yaml};
use log::log;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // Same as previous examples...
    let name = matches.subcommand_name();
    match name.unwrap() {
        "build-player"=>subcmd::build_player::test_output(),
        v => {
            println!("Not found command named *{0}*", v);
        }
    }
    // println!("{}", name.unwrap())
    // test();
}