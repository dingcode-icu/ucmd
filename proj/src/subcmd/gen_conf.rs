use log::{info, error};
use clap::ArgMatches;
use crate::subcmd::basecmd::BaseCmd;
use std::io::Write;

struct GenConf {
    conf_type: String,
    out_file: String
}


const BUILDERS_CONF: &str = include_str!("../static/build-player");

impl BaseCmd for GenConf{}

impl GenConf{
    fn new(ctype: String, outf: String) -> Self {
        GenConf{
            conf_type: ctype.to_string(),
            out_file: outf.to_string()
        }
    }

    fn run(&self){
        let _val = String::from("build-player");
        match &self.conf_type{
            _val=>{
                let mut f = std::fs::File::create(&self.out_file).expect(format!("create config file {} failed!", &self.out_file).as_str());
                f.write_all(BUILDERS_CONF.as_bytes()).expect(format!("write content to {} failed!", self.out_file).as_str());
                info!("Gen suc!");
                }
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let conf_support: Vec<&str> = vec!["build-player"];
    let target = subm.value_of("type");
    let outf = subm.value_of("output").unwrap();
    match target {
        None => {}
        Some(v) => {
            if !conf_support.contains(&v) {
                error!("{}", format!("Not support conf type {} yet! Do nothing", v));
                return;
            }
            let cmd = &GenConf::new(String::from(v), String::from(outf));
            cmd.run();
        }
    };
}