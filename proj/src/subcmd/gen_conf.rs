use log::{info, error};
use clap::ArgMatches;
use crate::subcmd::basecmd::BaseCmd;
use std::io::Write;

struct GenConf {
    conf_type: String,
    out_file: String,
}


const BUILDERS_TEMP: &str = include_str!("../static/env");
const ABMAP_TEMP: &str = include_str!("../static/build-ab");

impl BaseCmd for GenConf {
    fn run(&self) {
        let tar = self.conf_type.as_str();
        match tar {
            "build-player" | "env" => {
                let mut f = std::fs::File::create(&self.out_file).expect(format!("create config file {} failed!", &self.out_file).as_str());
                f.write_all(BUILDERS_TEMP.as_bytes()).expect(format!("write content to {} failed!", self.out_file).as_str());
                info!("Gen suc!");
            }
            "build-ab" => {
                const ab_f: &str = "build_ab.yaml";
                let mut f = std::fs::File::create(ab_f).expect(format!("create config file {} failed!", ab_f).as_str());
                f.write_all(ABMAP_TEMP.as_bytes()).expect(format!("write content to {} failed!", ab_f).as_str());
                info!("Gen suc!");
            }
            _ => {
                error!("{}", format!("Not support conf type {} yet! Do nothing", tar));
            }
        }
    }
}

impl GenConf {
    fn new(ctype: String, outf: String) -> Self {
        GenConf {
            conf_type: ctype.to_string(),
            out_file: outf.to_string(),
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let conf_support: Vec<&str> = vec!["env", "build-ab"];
    let target = subm.value_of("type");
    let o = subm.value_of("output").unwrap();
    match target {
        None => {}
        Some(v) => {
            let cmd = &GenConf::new(String::from(v), String::from(o));
            cmd.run();
        }
    };
}