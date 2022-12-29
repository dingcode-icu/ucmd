use crate::subcmd::basecmd::BaseCmd;
use std::io::Write;
use std::path::Path;
use log::info;
use rcmd_core::clap::{ArgMatches};

struct GenConf {
    proj_path: String
}


const BUILDERS_TEMP: &str = include_str!("../static/.ucmd"); 


impl BaseCmd for GenConf {
    fn run(&self) {
        let out_file = Path::new(self.proj_path.as_str()).join(".ucmd");
        let mut f = std::fs::File::create(&out_file).expect(format!("create config file {} failed!", out_file.display()).as_str());
        f.write_all(BUILDERS_TEMP.as_bytes()).expect(format!("write content to {} failed!", &out_file.display()).as_str());
        info!("Gen suc!");
    }
}

impl GenConf {
    fn new(p: String) -> Self {
        GenConf {
            proj_path: p
        }
    }
} 


pub fn handle(subm: &ArgMatches) {
    let cur_dir = std::env::current_dir().unwrap();
    let cur_path = cur_dir.to_str().unwrap().to_string();
    let proj_path = subm.value_of("path").unwrap_or_else(||cur_path.as_str());
    //todo chk proj type 
    let cmd = &GenConf::new(proj_path.to_string());
    cmd.run();
}

#[test]
fn test_init(){
    use std::env;
    let cur = env::current_dir().unwrap();
    let test_path = Path::new(cur.to_str().unwrap()).parent().unwrap().join("test");
    GenConf::new(test_path.display().to_string()).run();
    
}