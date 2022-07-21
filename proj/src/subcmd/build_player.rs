use crate::subcmd::basecmd::{BaseCmd, HookSupport, BuildType};
use rcmd_core::{ArgMatches, Log::debug};
use std::{process::exit};
use rcmd_core::Ex::yaml_rust::Yaml;

struct BuildPlayer {
    ///env配置
    build_config: Yaml,
    ///目标平台
    platform: String,
    ///
    root_path: String
}

impl BaseCmd for BuildPlayer {
    fn run(&self) {
        let args =  self.build_config["args"].as_str().unwrap().to_string() ;
        let hook_args = self.build_config["hook_args"].as_str().unwrap().to_string();
        // before hook
        let bf_p = vec![args.to_string(), hook_args.to_string()];
        self.execute_hook(HookSupport::BeforeBinBuild, &bf_p);
        // bin execute
        let suc = self.gen_bin(&self.build_config, &self.platform,  if self.platform == "ios" { BuildType::Ios } else {BuildType::Android}, hook_args.as_str());
        if !suc {
            exit(2);
        }
        // after hook
        let base = &self.build_config;
        let plat = self.platform.to_string();
        let cfg = &base[plat.as_str()];
        let is = cfg["path"].is_badvalue() || cfg["path"].is_null();
        let af_p = if is == false {
            vec![plat, cfg["path"].as_str().unwrap().to_string(), base["unity_proj"].as_str().unwrap().to_string()]
        } else {
            vec![plat, "".to_string(), base["unity_proj"].as_str().unwrap().to_string()]
        };
        self.execute_hook(HookSupport::AfterBinBuild, &af_p);
    }
}

impl BuildPlayer {
    fn new(config: &str, platform: String) -> Self {
        let p = Path::new(config).parent().unwrap();
        debug!("build_player new {}", p.to_str().unwrap());
        BuildPlayer {
            build_config: BuildPlayer::parse_yaml(config),
            platform,
            root_path:p.to_str().unwrap().to_string()
        }
    }
}


pub fn handle(subm: &ArgMatches) {
    let target = subm.value_of("platform");
    match target{
        None => {}
        Some(_) => {}
    }
    let conf = subm.value_of("config").unwrap();     //这里其实也不用match了 clap的require参数不符合clap就会过滤掉
    let cmd = &BuildPlayer::new(conf, target.unwrap().to_string());
    cmd.run();
}

#[test]

fn test_buildplayer() {
    use std::env;
    let conf = env::current_exe().unwrap()
    let cmd = &BuildPlayer::new(conf, target.unwrap().to_string());
    cmd.run();
}
