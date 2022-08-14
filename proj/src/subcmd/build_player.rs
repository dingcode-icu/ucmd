use log::info;
use rcmd_core::clap::ArgMatches;

use crate::subcmd::basecmd::{BaseCmd, HookSupport};
use std::{process::exit, path::Path};

use super::BuildType;

struct BuildPlayer {
    ///目标平台
    platform: String,
    ///工程路径
    proj_path: String
}

impl BaseCmd for BuildPlayer {
    fn run(&self) {
        info!("build-player in ->{}", self.proj_path);
        let conf_file = Path::new(&self.proj_path).join(".ucmd"); 
        let build_config = BuildPlayer::parse_yaml(conf_file.to_str().unwrap());
        let ucmdex_args = build_config["ex_args"].as_str().unwrap();
        
        // before hook
        let bf_p = vec![ucmdex_args.to_string()];
        self.execute_hook(self.proj_path.as_str(), HookSupport::BeforeBinBuild, &bf_p);
        // bin execute
        let suc = self.gen_target(self.proj_path.as_str(), &build_config, &self.platform,  if self.platform == "ios" { BuildType::Ios } else {BuildType::Android}, "");
        if !suc {
            exit(2);
        }
        // after hook
        self.execute_hook(self.proj_path.as_str(), HookSupport::AfterBinBuild, &bf_p);
    }
}

impl BuildPlayer {
    fn new(path: &str, platform: String) -> Self {
        BuildPlayer {
            platform,
            proj_path:path.to_string()
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let target = subm.value_of("platform");
    match target{
        None => {}
        Some(_) => {}
    }

    let cur_dir = std::env::current_dir().unwrap();
    let cur_path = cur_dir.to_str().unwrap();
    let proj_path = subm.value_of("path").unwrap_or_else(||cur_path);     //这里其实也不用match了 clap的require参数不符合clap就会过滤掉
    let cmd = &BuildPlayer::new(proj_path, target.unwrap().to_string());
    cmd.run();
}

#[test]
fn test_buildplayer() {
    let proj_path = std::env::current_dir().unwrap()
                    .parent().unwrap()
                    .join("test");
    let cmd = &BuildPlayer::new(proj_path.to_str().unwrap(), "ios".to_string());
    // cmd.run();
    
    cmd.execute_hook(proj_path.to_str().unwrap() , HookSupport::BeforeBinBuild, &vec!["a".to_string(), "b".to_string()])
}
