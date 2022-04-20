use std::io::Read;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, Display};
use std::fs;
use rcmd_core::clap::YamlLoader;
use rcmd_core::Ex::yaml_rust::Yaml;
use rcmd_core::Log::{debug, error, info};
use rcmd_core::util;

#[derive(Debug)]
pub enum HookSupport {
    ///gen ab
    BeforeGenAb,
    AfterGenAb,
    ///gen unity
    BeforeBinBuild,
    AfterBinBuild,
}

#[derive(Debug)]
pub enum BuildType {
    Android,
    Ios,
    Ab,
}

impl Display for BuildType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            BuildType::Android => { write!(f, "android") }
            BuildType::Ios => { write!(f, "ios") }
            BuildType::Ab => { write!(f, "ab") }
        }
    }
}

impl fmt::Display for HookSupport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


pub(crate) trait BaseCmd {
    ///检查env.yaml参数
    fn check_env_file(path: &str) -> bool {
        let logout = || {
            error!("env require *config* params to input the system env, check it！");
                false
        };
        let is_e = fs::metadata(path).is_ok();
        if is_e {
            if !fs::metadata(path).unwrap().is_file() {
                logout();
            }
        } else {
            logout();
        }
        true
    }

    ///检查platform参数
    fn check_support_platform(plat: &str) -> bool {
        let plat_support: Vec<&str> = vec!["android", "ios"];
        match plat_support.contains(&plat){
            true => {true}
            false => {false}
        }
    }

    ///加载环境配置，标准格式可通过gen_config生成
    fn parse_yaml(conf: &str) -> Yaml {
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        let doc = &docs[0];
        return doc.to_owned();
    }

    fn parse_json(conf: &str) -> serde_json::Value{
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        serde_json::from_str(&contents).unwrap()
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
            let (iss, ret) = util::shcmd::run_sh(&String::from(h_path.to_str().unwrap()), args);
            if iss {
                info!("{}", ret);
                return;
            }
            if ret.len() > 0 {
                error!("hook error:{}", ret);
            }
            return;
        }
        debug!("{}", format!("No hook {}", &h_path.to_str().unwrap()));
    }

    ///执行unity cmd
    fn gen_unity_asset(&self, config: &Yaml, plat: &str, build_type: BuildType, ex_cmd: &str) -> bool {
        let base = config;
        let cfg = &base[plat];
        let args_base = base["args"].as_str().unwrap();
        let cmd = base["unity_bin"].as_str().unwrap();
        let logfile = base["log_output_path"].as_str().unwrap().to_string() + util::get_strfmt_timestr("%Y%m%T%d").as_str() + ".log";
        let unity_proj = base["unity_proj"].as_str().unwrap();
        let method = base[build_type.to_string().as_str()]["method"].as_str().unwrap();
        let args_str = &format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        -logfile {logfile} \
        -targetPlatform:{plat} \
        {ex_cmd}",

                                args_base = args_base,
                                method = method,
                                unity_proj = unity_proj,
                                logfile = logfile,
                                plat = plat,
                                ex_cmd = ex_cmd
        );
        let args = args_str.split(" ").collect::<Vec<&str>>();
        info!("Gen the unity asset...");
        info!("Full unity command is {} {}",cmd, &args.join(" "));
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
        } else {
            error!("Check the error in unity output logfile! ")
        }
        return false;
    }

    fn run(&self);
}