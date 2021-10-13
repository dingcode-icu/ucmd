use crate::subcmd::basecmd::{BaseCmd, HookSupport};
use yaml_rust::Yaml;
use clap::ArgMatches;
use log::{*};
use std::fs;
use rcmd_core::util;
use std::time::Duration;
use std::ops::Index;

struct BuildPlayer {
    build_config: Yaml,
    platform: String,
}

impl BaseCmd for BuildPlayer {}

impl BuildPlayer {
    fn new(config: &str, platform: String) -> Self {
        BuildPlayer {
            build_config: BuildPlayer::parse_config(config),
            platform,
        }
    }

    fn gen_unity_asset(&self) -> bool {
        let base = &self.build_config;
        let plat = self.platform.as_str();
        let cfg = &base[plat];
        let unity_bin = base["unity_bin"].as_str();
        let logfile = base["log_output_path"].as_str().unwrap().to_string() + util::get_strfmt_timestr("%Y%m%T%d").as_str() + ".log";
        let cmd = &unity_bin.unwrap().to_string();
        let args_str = &format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        -logfile {logfile}",
                                args_base = base["args"].as_str().unwrap(),
                                method = cfg["method"].as_str().unwrap(),
                                unity_proj = base["unity_proj"].as_str().unwrap(),
                                logfile = logfile
        );
        let args = args_str.split(" ").collect::<Vec<&str>>();
        info!("Gen the unity asset...");
        info!("It will cost a long time \n\
                  Enter the following command to check the process...\n\
                  +++++++++++++++++++++++++++++++++\n\
                  tail -f {logfile}\n\
                  +++++++++++++++++++++++++++++++++\n\
                  ", logfile = logfile);
        let (suc, ret) = util::shcmd::run_sh(cmd, &args);
        if suc {
            info!("Gen unity asset success!");
            return true;
        }
        info!("Gen unity asset failed!");
        if ret.len() > 0 {
            error!("{}", ret);
        }
        else{
            error!("Check the error in unity output logfile! ")
        }
        return false;
    }

    fn run(&self) {
        let args = self.build_config["args"].as_str().unwrap();
        // before hook
        let bf_p = vec![args];
        self.execute_hook(HookSupport::BeforeGenUnity, &bf_p);
        let suc = self.gen_unity_asset();
        if !suc {
            return;
        }
        // after hook
        let base = &self.build_config;
        let plat = self.platform.as_str();
        let cfg = &base[plat];
        let is = cfg["path"].is_badvalue();
        let af_p = if is == false {
            vec![plat, cfg["path"].as_str().unwrap(), base["unity_proj"].as_str().unwrap()]
        } else {
            vec![plat, "", base["unity_proj"].as_str().unwrap()]
        };
        self.execute_hook(HookSupport::AfterGenUnity, &af_p);
    }
}

pub fn handle(subm: &ArgMatches) {
    let plat_support: Vec<&str> = vec!["android", "ios"];
    let target = subm.value_of("platform");
    let conf = subm.value_of("config").unwrap();     //这里其实也不用match了 require不符合标准clap就会过滤掉
    match target {
        None => {}
        Some(v) => {
            if !plat_support.contains(&v) {
                error!("Not support platform {} yet! Do nothing", v);
                return;
            }

            let logout = || {
                error!("build-player require *config* params to input the system env, check it！");
            };
            let is_e = fs::metadata(conf).is_ok();
            if is_e {
                if !fs::metadata(conf).unwrap().is_file() {
                    logout();
                    return;
                }
            } else {
                logout();
                return;
            }
            let cmd = &BuildPlayer::new(conf, v.to_string());
            cmd.run();
        }
    };
}