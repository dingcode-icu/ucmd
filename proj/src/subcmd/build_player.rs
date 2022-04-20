use crate::subcmd::basecmd::{BaseCmd, HookSupport, BuildType};
use std::fs;
use rcmd_core::{ArgMatches, util};
use std::time::Duration;
use std::ops::Index;
use std::process::exit;
use rcmd_core::Ex::yaml_rust::Yaml;

struct BuildPlayer {
    ///env配置
    build_config: Yaml,
    ///目标平台
    platform: String,
}

impl BaseCmd for BuildPlayer {
    fn run(&self) {
        let args = self.build_config["args"].as_str().unwrap();
        let hook_args = self.build_config["hook_args"].as_str().unwrap();
        // before hook
        let bf_p = vec![args, hook_args];
        self.execute_hook(HookSupport::BeforeBinBuild, &bf_p);
        let suc = self.gen_unity_asset(&self.build_config, &self.platform,  if self.platform == "ios" { BuildType::Ios } else {BuildType::Android}, hook_args);
        if !suc {
            exit(2);
        }
        // after hook
        let base = &self.build_config;
        let plat = self.platform.as_str();
        let cfg = &base[plat];
        let is = cfg["path"].is_badvalue() || cfg["path"].is_null();
        let af_p = if is == false {
            vec![plat, cfg["path"].as_str().unwrap(), base["unity_proj"].as_str().unwrap()]
        } else {
            vec![plat, "", base["unity_proj"].as_str().unwrap()]
        };
        self.execute_hook(HookSupport::AfterBinBuild, &af_p);
    }
}

impl BuildPlayer {
    fn new(config: &str, platform: String) -> Self {
        BuildPlayer {
            build_config: BuildPlayer::parse_yaml(config),
            platform,
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let target = subm.value_of("platform");
    match target{
        None => {}
        Some(_) => {}
    }
    let conf = subm.value_of("config").unwrap();     //这里其实也不用match了 require不符合标准clap就会过滤掉
    let cmd = &BuildPlayer::new(conf, target.unwrap().to_string());
    cmd.run();
}