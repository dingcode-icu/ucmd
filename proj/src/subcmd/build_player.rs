use crate::subcmd::basecmd::{BaseCmd, HookSupport};
use crate::util;
use yaml_rust::Yaml;
use clap::ArgMatches;
use log::{*};
use std::fs;

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
        let platcfg = &base[plat];
        let unity_bin = base["unity_bin"].as_str();
        let logfile = base["log_output_path"].as_str().unwrap().to_string() + util::get_strfmt_timestr("%Y%m%T%d").as_str() + ".log";
        let cmd = &unity_bin.unwrap().to_string();
        let args_str = &format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        -logfile {logfile}",
                                args_base = base["args"].as_str().unwrap(),
                                method = platcfg["method"].as_str().unwrap(),
                                unity_proj = base["unity_proj"].as_str().unwrap(),
                                logfile = logfile
        );
        let args = args_str.split(" ").collect::<Vec<&str>>();
        info!("Gen the unity asset...");
        info!("It will cost a long time \n\
                  Input down cmd to check the process...\n\
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
        error!("{}", ret);
        return false;
    }

    fn run(&self) {
        let args = self.build_config["args"].as_str().unwrap();
        let params = vec![args];
        self.execute_hook(HookSupport::BeforeGenUnity, &params);
        let suc = self.gen_unity_asset();
        if !suc {
            return;
        }
        self.execute_hook(HookSupport::AfterGenUnity, &params);
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

            let is_e = fs::metadata(conf).is_ok();
            if is_e {
                if !fs::metadata(conf).unwrap().is_file() {
                    error!("build-player require *config* params to input the system env, check it！");
                    return;
                }
            }
            let cmd = &BuildPlayer::new(conf, v.to_string());
            cmd.run();
        }
    };
}