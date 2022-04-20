use std::io::Read;
use std::path::Path;
use std::env;
use std::fmt;
use std::fmt::{Formatter, Display};
use std::fs;
use rcmd_core::Log::warn;
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

#[derive(Debug, Copy, Clone)]
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



trait BinCmd {
    fn build_ab(&self) -> Vec<String>;
    fn build_player(&self) -> Vec<String>;
}

struct CocosCreatorBinV2<'a>{
    config: &'a Yaml, 
    plat: &'a str, 
    build_type: BuildType, 
    ex_cmd: &'a str, 
}
impl<'a> CocosCreatorBinV2<'a> {
    fn new(config: &'a Yaml, plat: &'a str, build_type: BuildType, ex_cmd: &'a str) ->Self{
        CocosCreatorBinV2{
            config,
            plat,
            build_type,
            ex_cmd,
        }
    }
}

struct UnityBin<'a> {
    config: &'a Yaml, 
    plat: &'a str, 
    build_type: BuildType, 
    ex_cmd: &'a str
}
impl<'a> UnityBin<'a> {
    fn new(config: &'a Yaml, plat: &'a str, build_type: BuildType, ex_cmd: &'a str) ->Self{
        UnityBin{
            config,
            plat,
            build_type,
            ex_cmd,
        }
    }
}


impl UnityBin<'_>{
    pub fn base_cmd(&self) ->Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let logfile = config["log_output_path"].as_str().unwrap().to_string() + util::get_strfmt_timestr("%Y%m%T%d").as_str() + ".log";
        let unity_proj = config["asset_proj"].as_str().unwrap();
        let method = config[self.build_type.to_string().as_str()]["method"].as_str().unwrap();
        let args_str = format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        -logfile {logfile} \
        -targetPlatform:{plat} \
        {ex_cmd}",

                                args_base = args_base,
                                method = method,
                                unity_proj = unity_proj,
                                logfile = logfile,
                                plat = self.plat,
                                ex_cmd = self.ex_cmd
        );
        let args:Vec<String> = args_str.split(" ").map(|v|v.to_string()).collect();
        info!("Gen the unity asset...");
        // info!("Full unity command is {}", &args.join(" "));
        args
    }
}

impl CocosCreatorBinV2<'_> {
    pub fn base_cmd(&self) ->Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let logfile = config["log_output_path"].as_str().unwrap().to_string() + util::get_strfmt_timestr("%Y%m%T%d").as_str() + ".log";
        let unity_proj = config["asset_proj"].as_str().unwrap();
        let method = config[self.build_type.to_string().as_str()]["method"].as_str().unwrap();
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
                                plat = self.plat,
                                ex_cmd = self.ex_cmd
        );
        let args:Vec<String> = args_str.split(" ").map(|v|v.to_string()).collect();
        info!("Gen the unity asset...");
        // info!("Full unity command is {}", &args.join(" "));
        args
    }
}



impl BinCmd for UnityBin<'_> {
    fn build_ab(&self) -> Vec<String> {
        self.base_cmd()
    }

    fn build_player(&self) ->  Vec<String> {
        self.base_cmd()
    }
}


impl BinCmd for CocosCreatorBinV2<'_>{
    fn build_ab(&self) -> Vec<String> {
        self.base_cmd()
    }

    fn build_player(&self) ->  Vec<String> {
        self.base_cmd()
    }
}


///查看打包工具的类型
fn chk_bintype(bin: &str){

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
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
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

    ///执行bin cmd
    fn gen_bin(&self, config: &Yaml, plat: &str, build_type: BuildType, ex_cmd: &str) -> bool {
        let cmd = config["bin"].as_str().unwrap();
        let cmd_type = config["bin_type"].as_str().unwrap(); 

        let mut args= vec![];
        let p_type:PlayerType = cmd_type.to_string().into();
        while p_type != PlayerType::UnKnown {
            if p_type == PlayerType::Unity {
                args = UnityBin::new(config, plat, build_type, ex_cmd).base_cmd();
            }
            else if p_type == PlayerType::CocosCreatorv2 {
                args = CocosCreatorBinV2::new(config, plat, build_type, ex_cmd).base_cmd();
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