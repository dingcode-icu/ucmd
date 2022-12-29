use crate::subcmd::players::cocosv2::CocosCreatorV2Proj;
use crate::subcmd::players::unity::UnityProj;
use log::error;
use log::info;
use log::warn;
use rcmd_core::clap::YamlLoader;
use rcmd_core::util;
use rcmd_core::yaml_rust::Yaml;
use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::BuildType;

#[derive(Debug)]
pub enum HookSupport {
    //game resource prepare
    PreSources,
    //binary of game engine build command
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
    CocosCreatorv2,
}

impl From<PlayerType> for String {
    fn from(p: PlayerType) -> Self {
        match p {
            PlayerType::Unity => "unity".into(),
            PlayerType::CocosCreatorv2 => "cocoscreator_v2".into(),
            _ => "unknown".into(),
        }
    }
}

impl From<String> for PlayerType {
    fn from(s: String) -> Self {
        match s {
            s if s == "unity" => PlayerType::Unity,
            s if s == "cocoscreator_v2" => PlayerType::CocosCreatorv2,
            _ => PlayerType::UnKnown,
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
        match plat_support.contains(&plat) {
            true => true,
            false => false,
        }
    }

    ///加载yaml配置
    fn parse_yaml(conf: &str) -> Yaml {
        let mut file = std::fs::File::open(conf).expect("Read yml config file error!");
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        drop(file);
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        if docs.len() == 0 {
            panic!("Parse yml config raise error!")
        }
        let doc = &docs[0];
        return doc.to_owned();
    }

    ///加载json配置
    fn parse_json(conf: &str) -> serde_json::Value {
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        serde_json::from_str(&contents).unwrap()
    }

    ///执行hook,hook类似git的hook机制，在构建关键节点执行本地脚本
    fn execute_hook(&self, proj_path: &str, hook: HookSupport, args: &Vec<String>) {
        let h_name = format!("{:?}", hook);
        let h_path = Path::new(proj_path).join(".ucmd_hook").join(&h_name);
        info!(
            "{}",
            format!("Check execute hook:{}", &h_path.to_str().unwrap())
        );
        let mf = fs::metadata(&h_path);
        if mf.is_ok() {
            info!(
                "{}",
                format!("found the hook file {}", &h_path.to_str().unwrap())
            );
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
        info!("{}", format!("No hook {}", &h_path.to_str().unwrap()));
    }

    ///根据配置检查默认内置参数
    fn get_hook_exargs(
        &self,
        build_config: &Yaml,
        build_path: &PathBuf,
        build_type: BuildType,
        ex_args: Option<&str>,
    ) -> String {
        let mut m_ucmdex_args: String = String::from("");
        let ucmdex_args =
            if build_config["ex_args"].is_badvalue() || build_config["ex_args"].is_null() {
                None
            } else {
                Some(build_config["ex_args"].as_str().unwrap())
            };
        if ucmdex_args.is_some() {
            m_ucmdex_args += ucmdex_args.unwrap();
        }
        m_ucmdex_args += ex_args.or(Some("")).unwrap();
        m_ucmdex_args += format!(" -_outputPath:{}", build_path.display()).as_str();
        m_ucmdex_args += format!(" -_targetPlatform:{}", build_type).as_str();
        m_ucmdex_args
    }

    ///生成build文件夹初始化
    fn gen_build_path(&self, proj_path: PathBuf, build_type: BuildType) -> PathBuf {
        //append ucmndex_args
        let build_path = self.get_build_path(proj_path, build_type.to_string());
        //output path
        if !Path::new(build_path.as_path()).is_dir() {
            std::fs::create_dir_all(build_path.as_path())
                .expect("create dir <.ucmd_build> failed!");
        }
        build_path
    }

    ///build路径
    fn get_build_path(&self, proj_path: PathBuf, subname: String) -> PathBuf {
        let build_path = proj_path.join(".ucmd_build").join(subname);
        build_path
    }

    ///执行bin cmd
    fn gen_target(
        &self,
        proj_path: &str,
        config: &Yaml,
        build_type: BuildType,
        build_path: &str,
        // ex_cmd: &str,
        ucmdex_args: & mut Vec<String>
    ) -> bool {
        self.gen_build_path(Path::new(proj_path).to_path_buf(), build_type);
        let cmd = config["bin"].as_str().unwrap();
        let cmd_type = config["bin_type"].as_str().or(Some("unity")).unwrap();

        let mut args = vec![];
        let p_type: PlayerType = cmd_type.to_string().into();
        if p_type != PlayerType::UnKnown {
            if p_type == PlayerType::Unity {
                args = UnityProj::new(proj_path, config, build_type, build_path, ucmdex_args).base_cmd();
            } else if p_type == PlayerType::CocosCreatorv2 {
                args = CocosCreatorV2Proj::new(proj_path, config, build_type, build_path, ucmdex_args)
                    .base_cmd();
            } else {
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
            error!("Raise error in binary build! ")
        }
        return false;
    }

    ///trait of run 子结构必须自行实现
    fn run(&self);
}
