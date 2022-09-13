use log::{info, error};
use rcmd_core::clap::ArgMatches;

use crate::subcmd::basecmd::{BaseCmd, HookSupport};
use std::{path::Path, process::exit};

use super::BuildType;

struct BuildPlayer<'a> {
    ///配置文件
    conf_file: Option<&'a str>,
    ///构建目标
    build_type: BuildType,
    ///工程路径
    proj_path: String,
    ///额外的参数
    ex_args: Option<&'a str>,
}

impl<'a> BaseCmd for BuildPlayer<'_> {
    fn run(&self) {
        info!("build-player in ->{}", self.proj_path);
        let conf_file = if self.conf_file.is_none() {
            Path::new(&self.proj_path).join(".ucmd")
        } else {
            Path::new(&self.proj_path).join(&self.conf_file.unwrap())
        };
        println!("ucmd config file is {}", conf_file.display());
        let build_config = BuildPlayer::parse_yaml(conf_file.to_str().unwrap());
        //chk build path
        let build_path = self.gen_build_path(self.build_type);
        //hook args
        let hook_args = self.get_hook_exargs(&build_config, &build_path, self.build_type, self.ex_args);
        let hook_argvec = vec![hook_args.to_string()];
        self.execute_hook(
            self.proj_path.as_str(),
            HookSupport::BeforeBinBuild,
            &hook_argvec,
        );
        // bin execute
        let suc = self.gen_target(
            self.proj_path.as_str(),
            &build_config,
            self.build_type,
            build_path.display().to_string().as_str(),
            hook_args.as_str(),
        );
        if !suc {
            exit(2);
        }
        // after hook
        self.execute_hook(
            self.proj_path.as_str(),
            HookSupport::AfterBinBuild,
            &hook_argvec,
        );
    }
}

impl<'a> BuildPlayer<'a> {
    fn new(
        path: &str,
        build_type: BuildType,
        conf_file: Option<&'a str>,
        ex_args: Option<&'a str>,
    ) -> Self {
        BuildPlayer {
            conf_file,
            build_type,
            proj_path: path.to_string(),
            ex_args,
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let target = subm.value_of("platform");
    let config = subm.value_of("config");
    let ex_args = subm.value_of("ex_args");
    let cur_dir = std::env::current_dir().unwrap();
    let cur_path = cur_dir.to_str().unwrap();
    let proj_path = subm.value_of("path").unwrap_or_else(|| cur_path); //这里其实也不用match了 clap的require参数不符合clap就会过滤掉
    //chk buildtype support 
    let bt = BuildType::from(target.unwrap().to_string());

    match bt{
        BuildType::UnSupport=>{
            error!("ucmd build target <{}> not support yet!", target.unwrap());
            std::process::exit(1)
        }
        _=>{}
    }

    let cmd = &BuildPlayer::new(proj_path, bt, config, ex_args);
    cmd.run();
}

#[test]
fn test_buildplayer() {
    let proj_path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("test");
}
