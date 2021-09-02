use crate::subcmd::basecmd::BaseCmd;
use crate::util;
use yaml_rust::Yaml;
use clap::ArgMatches;
use log::{*};

struct BuildPlayer {
    build_config: Yaml,
    platform: String,
    isr: bool,
}

impl BaseCmd for BuildPlayer {}

impl BuildPlayer {
    fn new(config: &str, platform: String, isr: bool) -> Self {
        BuildPlayer {
            build_config: BuildPlayer::parse_config(config),
            platform,
            isr,
        }
    }

    fn gen_unity_asset(&self) -> bool {
        let base = &self.build_config;
        let plat = self.platform.as_str();
        let platcfg = &base[plat];
        let unity_bin = base["unity_bin"].as_str();
        let logfile = base["log_output_path"].as_str().unwrap().to_string() + util::get_Ymdt_timestr().as_str() + ".log";
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
        error!(ret);
        return false;
    }

    fn run(&self) {
        let suc = self.gen_unity_asset();
        if !suc {
            return;
        }
    }
}


pub fn handle(subm: &ArgMatches) {
    let isr = subm.is_present("release");
    let PLAT_SUPPORT: Vec<&str> = vec!["android", "ios"];
    let target = subm.value_of("platform");
    let conf = subm.value_of("config").unwrap();     //这里其实也不用match了 reuqire不符合标准clap就会过滤掉
    match target {
        None => {}
        Some(v) => {
            if !PLAT_SUPPORT.contains(&v) {
                println!("Not support platform {} yet! Do nothing", v);
                return;
            }
            let cmd = &BuildPlayer::new(conf, v.to_string(), isr);
            cmd.run();
        }
    };
}