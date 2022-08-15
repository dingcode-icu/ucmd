use log::info;
use rcmd_core::yaml_rust::Yaml;

use crate::subcmd::BuildType;
use super::BinCmd;

pub struct CocosCreatorV2Proj<'a>{
    proj_path:&'a str,
    config: &'a Yaml, 
    plat: &'a str, 
    build_path: &'a str, 
    ex_cmd: &'a str, 
}

impl<'a> CocosCreatorV2Proj<'a> {
    pub fn new(proj_path:&'a str, config: &'a Yaml, plat: &'a str, build_path: &'a str, ex_cmd: &'a str) ->Self{
        CocosCreatorV2Proj{
            proj_path,
            config,
            plat,
            build_path,
            ex_cmd,
        }
    }
}

impl CocosCreatorV2Proj<'_> {
    pub fn base_cmd(&self) ->Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let cocos_proj = self.proj_path;
        let method = "unknow";
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
        args
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
