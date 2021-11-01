use log::{info, error};
use clap::ArgMatches;
use crate::subcmd::basecmd::BaseCmd;
use std::io::Write;

struct GenConf {
    conf_type: String,
    out_file: String
}


const BUILDERS_CONF: &str = include_str!("../static/env");

impl BaseCmd for GenConf{
    fn run(&self){
        let v= String::from("build-player");
        match &self.conf_type{
            v =>{
                let mut f = std::fs::File::create(&self.out_file).expect(format!("create config file {} failed!", &self.out_file).as_str());
                f.write_all(BUILDERS_CONF.as_bytes()).expect(format!("write content to {} failed!", self.out_file).as_str());
                info!("Gen suc!");
                }
        }
    }
}

impl GenConf{
    fn new(ctype: String, outf: String) -> Self {
        GenConf{
            conf_type: ctype.to_string(),
            out_file: outf.to_string()
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let conf_support: Vec<&str> = vec!["env"];
    let target = subm.value_of("type");
    let o = subm.value_of("output").unwrap();
    match target {
        None => {}
        Some(v) => {
            if !conf_support.contains(&v) {
                error!("{}", format!("Not support conf type {} yet! Do nothing", v));
                return;
            }
            let cmd = &GenConf::new(String::from(v), String::from(o));
            cmd.run();
        }
    };
}