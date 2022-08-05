use std::io::Read;
use std::path::Path;
use std::env;
use std::fmt;
use std::fmt::{Formatter};
use std::fs;
use rcmd_core::Log::warn;
use rcmd_core::clap::YamlLoader;
use rcmd_core::Ex::yaml_rust::Yaml;
use rcmd_core::Log::{debug, error, info};
use rcmd_core::util;

use crate::subcmd::players::unity::CocosCreatorV2Proj;
use crate::subcmd::players::unity::UnityProj;

use super::BuildType;

#[derive(Debug)]
pub enum HookSupport {
    ///gen ab
    BeforeGenAb,
    AfterGenAb,
    ///gen unity
    BeforeBinBuild,
    AfterBinBuild,
}

impl fmt::Display for HookSupport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, PartialEq)]
enum PlayerType {
   UnKnown,
   Unity,
   CocosCreatorv2 
}

impl From<PlayerType> for String{
    fn from(p: PlayerType) -> Self {
        match p {
            PlayerType::Unity => "unity".into(),
            PlayerType::CocosCreatorv2 => "cocoscreator_v2".into(),
            _=>"unknown".into(),
        }
    }
}

impl From<String> for PlayerType {
    fn from(s: String) -> Self {
        match s {
            s if s == "unity" => PlayerType::Unity,
            s if s == "cocoscreator_v2" => PlayerType::CocosCreatorv2,
            _ => PlayerType::UnKnown
        }
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

    ///加载yaml配置
    fn parse_yaml(conf: &str) -> Yaml {
        let mut file = std::fs::File::open(conf).expect("Read yml config file error!");
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        drop(file);
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        if docs.len() == 0{
            panic!("Parse yml config raise error!")
        }
        let doc = &docs[0];
        return doc.to_owned();
    }

    ///加载json配置
    fn parse_json(conf: &str) -> serde_json::Value{
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        serde_json::from_str(&contents).unwrap()
    }

    ///执行hook,hook类似git的hook机制，在构建关键节点执行本地脚本
    fn execute_hook(&self, hook: HookSupport, args: &Vec<String>) {
        let exe = env::current_exe().unwrap();
        let pwd = exe.parent().unwrap();
        let h_name = format!("{:?}", hook);
        let h_path = Path::join(pwd, ".ucmd_hook").join(&h_name);
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

    ///执行bin cmd
    fn gen_target(&self, proj_path:&str, config: &Yaml, plat: &str, build_type: BuildType, ex_cmd: &str) -> bool {
        let cmd = config["bin"].as_str().unwrap();
        let cmd_type = config["bin_type"].as_str().or(Some("unity")).unwrap(); 

        let mut args= vec![];
        let p_type:PlayerType = cmd_type.to_string().into();
        if p_type != PlayerType::UnKnown { 
            if p_type == PlayerType::Unity {
                args = UnityProj::new(proj_path, config, plat, build_type, ex_cmd).base_cmd();
            }
            else if p_type == PlayerType::CocosCreatorv2 {
                args = CocosCreatorV2Proj::new(proj_path, config, plat, build_type, ex_cmd).base_cmd();
            }
            else {
                warn!("[basecmd] not found the player type {:?}", cmd_type);
                std::process::exit(0)    
            }
        }

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

    ///trait of run 子结构必须自行实现
    fn run(&self);
}