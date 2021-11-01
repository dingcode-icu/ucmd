use crate::subcmd::basecmd::{BaseCmd, HookSupport, BuildType};
use std::process::exit;
use yaml_rust::Yaml;
use rcmd_core::{util, ArgMatches};
use log::{*};

///构建asset bundle命令
struct BuildAb {
    ///env配置
    build_config: Yaml,
    ///目标平台
    platform: String,
    ///是否只包含prefab
    is_p: bool,
    ///要打包prefab路径
    paths:Vec<String>,
}

impl BaseCmd for BuildAb {
    fn run(&self) {
        let args = self.build_config["args"].as_str().unwrap();
        let hook_args = self.build_config["hook_args"].as_str().unwrap();
        // before hook
        let bf_p = vec![args, hook_args];
        self.execute_hook(HookSupport::BeforeGenAb, &bf_p);
        let suc = self.gen_unity_asset(&self.build_config, self.platform.as_str(), BuildType::Ab);
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
    fn new(config: &str, platform: String, is_only_prefab: bool, paths: Vec<String>) -> Self{
        BuildAb {
            build_config: BuildAb::parse_config(config),
            platform,
            is_p: is_only_prefab,
            paths
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let is_p = subm.value_of("is_only_prefab").unwrap_or("false");
    let target = subm.value_of("platform");
    let conf = subm.value_of("config").unwrap();
    let paths = subm.value_of("relpaths").unwrap().split(",").collect::<Vec<&str>>();
    let mut paths_S = vec![];
    for p in paths {
        paths_S.push(String::from(p));
    }
    let cmd = &BuildAb::new(conf, target.unwrap().to_string(), is_p == "true", paths_S);
    cmd.run();
}