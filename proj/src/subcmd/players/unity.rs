use rcmd_core::{Ex::yaml_rust::Yaml, util, Log::info};

use crate::subcmd::BuildType;

trait BinCmd {
    fn build_ab(&self) -> Vec<String>;
    fn build_player(&self) -> Vec<String>;
}

pub struct CocosCreatorV2Proj<'a>{
    proj_path:&'a str,
    config: &'a Yaml, 
    plat: &'a str, 
    build_type: BuildType, 
    ex_cmd: &'a str, 
}
impl<'a> CocosCreatorV2Proj<'a> {
    pub fn new(proj_path:&'a str, config: &'a Yaml, plat: &'a str, build_type: BuildType, ex_cmd: &'a str) ->Self{
        CocosCreatorV2Proj{
            proj_path,
            config,
            plat,
            build_type,
            ex_cmd,
        }
    }
}

pub struct UnityProj<'a> {
    proj_path:&'a str,
    config: &'a Yaml, 
    plat: &'a str, 
    build_type: BuildType, 
    ex_cmd: &'a str
}
impl<'a> UnityProj<'a> {
    pub fn new(proj_path:&'a str, config: &'a Yaml, plat: &'a str, build_type: BuildType, ex_cmd: &'a str) ->Self{
        UnityProj{
            proj_path,
            config,
            plat,
            build_type,
            ex_cmd,
        }
    }
}


impl UnityProj<'_>{
    pub fn base_cmd(&self) ->Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let unity_proj = self.proj_path;
        let method = config["method"].as_str().unwrap();
        let args_str = format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        -targetPlatform:{plat} \
        {ex_cmd}",
                                args_base = args_base,
                                method = method,
                                unity_proj = unity_proj,
                                plat = self.plat,
                                ex_cmd = self.ex_cmd
        );
        let args:Vec<String> = args_str.split(" ").map(|v|v.to_string()).collect();
        info!("Gen the unity asset...");
        info!("Full unity command is \n{}", &args.join(" "));
        args
    }
}

impl CocosCreatorV2Proj<'_> {
    pub fn base_cmd(&self) ->Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let cocos_proj = self.proj_path;
        let method = config[self.build_type.to_string().as_str()]["method"].as_str().unwrap();
        let args_str = &format!("{args_base} \
        -executeMethod {method} \
        -projectPath {cocos_proj} \
        -targetPlatform:{plat} \
        {ex_cmd}",
                                args_base = args_base,
                                method = method,
                                cocos_proj = cocos_proj,
                                plat = self.plat,
                                ex_cmd = self.ex_cmd
        );
        let args:Vec<String> = args_str.split(" ").map(|v|v.to_string()).collect();
        info!("Gen the cocos asset...");
        // info!("Full unity command is {}", &args.join(" "));
        args
    }
}


impl BinCmd for UnityProj<'_> {
    fn build_ab(&self) -> Vec<String> {
        self.base_cmd()
    }

    fn build_player(&self) ->  Vec<String> {
        self.base_cmd()
    }
}


impl BinCmd for CocosCreatorV2Proj<'_>{
    fn build_ab(&self) -> Vec<String> {
        self.base_cmd()
    }

    fn build_player(&self) ->  Vec<String> {
        self.base_cmd()
    }
}
