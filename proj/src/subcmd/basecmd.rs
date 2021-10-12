extern crate yaml_rust;

use yaml_rust::{YamlLoader};
use std::io::Read;
use self::yaml_rust::Yaml;
use std::path::Path;
use std::env;
use log::{debug, warn, info, error};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::fs;
use rcmd_core::util;

#[derive(Debug)]
pub enum HookSupport {
    BeforeGenUnity,
    AfterGenUnity
}

impl fmt::Display for HookSupport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


pub(crate) trait BaseCmd {
    ///加载环境配置，标准格式可通过gen_config生成
    fn parse_config(conf: &str) -> Yaml {
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        let doc = &docs[0];
        return doc.to_owned();
    }

    ///执行hook,hook类似git的hook机制，在构建关键节点执行本地脚本
    fn execute_hook(&self, hook: HookSupport, args: &Vec<&str>) {
        let exe = env::current_exe().unwrap();
        let pwd = exe.parent().unwrap();
        let h_name = format!("{:?}", hook);
        let h_path = Path::join(pwd, "hook").join(&h_name);
        debug!("{}", format!("Check execute hook:{}", &h_path.to_str().unwrap()));
        let mf = fs::metadata(&h_path);
        if mf.is_ok() {
            info!("{}", format!("found the hook file {}", &h_path.to_str().unwrap()));
            let (iss, ret)  = util::shcmd::run_sh(&String::from(h_path.to_str().unwrap()), args);
            if iss {
                info!("{}", ret);
                return;
            }
            error!("hook error:{}", ret);
            return;
        }
        debug!("{}", format!("No hook {}", &h_path.to_str().unwrap()));
    }

    fn run(&self) {}
}