use crate::subcmd::basecmd::{BaseCmd, HookSupport, BuildType};
use std::process::exit;
use yaml_rust::Yaml;
use rcmd_core::{util, ArgMatches};
use log::{*};
use std::hash::Hash;
use std::collections::HashMap;
use serde_json::Value;


struct AbConfig {
    asset_paths: HashMap<String, String>,
}

///构建asset bundle命令
struct BuildAb {
    ///env配置
    build_config: Yaml,
    ///目标平台
    platform: String,
    ///bundle配置
    ab_config: Yaml,
}

impl BaseCmd for BuildAb {
    fn run(&self) {
        // before hook
        let args = self.build_config["args"].as_str().unwrap();
        let hook_args = self.build_config["hook_args"].as_str().unwrap();
        // before hook
        let bf_p = vec![args, hook_args];
        self.execute_hook(HookSupport::BeforeGenAb, &bf_p);
        // build list
        let o = self.ab_config["asset_paths"].as_hash().unwrap();
        let items: HashMap<&str, &str> = o.iter().map(|t| (t.0.as_str().unwrap(), t.1.as_str().unwrap())).collect();
        let mut ex_mcd = String::new();
        for i in items {
            let c1 = format!("{}={}|", i.0, i.1);
            ex_mcd += c1.as_str();
        }
        let suc = self.gen_unity_asset(&self.build_config, self.platform.as_str(), BuildType::Ab, format!("{} -abMap:{}", hook_args, ex_mcd).as_str());
        if !suc {
            exit(2);
        }
        // after hook
        let base = &self.build_config;
        let af_p = vec![self.platform.as_str(), "", base["unity_proj"].as_str().unwrap()];
        self.execute_hook(HookSupport::AfterGenAb, &af_p);
    }
}

impl BuildAb {
    fn new(platform: String, build_conf: &str, ab_conf: &str) -> Self {
        BuildAb {
            build_config: BuildAb::parse_yaml(build_conf),
            platform,
            ab_config: BuildAb::parse_yaml(ab_conf),
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let target = subm.value_of("platform");
    let ab_config = subm.value_of("ab_config").unwrap();
    let env = subm.value_of("env").unwrap();
    let cmd = &BuildAb::new(target.unwrap().to_string(), env, ab_config);
    cmd.run();
}