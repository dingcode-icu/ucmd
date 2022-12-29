use log::info;
use rcmd_core::yaml_rust::Yaml;

use super::BinCmd;
use crate::subcmd::BuildType;

pub struct CocosCreatorV2Proj<'a> {
    proj_path: &'a str,
    config: &'a Yaml,
    plat: BuildType,
    build_path: &'a str,
    ucmdex_args: &'a Vec<String>,
}

impl<'a> CocosCreatorV2Proj<'a> {
    pub fn new(
        proj_path: &'a str,
        config: &'a Yaml,
        plat: BuildType,
        build_path: &'a str,
        ucmdex_args: &'a Vec<String>,
    ) -> Self {
        CocosCreatorV2Proj {
            proj_path,
            config,
            plat,
            build_path,
            ucmdex_args,
        }
    }
}

impl CocosCreatorV2Proj<'_> {
    pub fn base_cmd(&self) -> Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let cocos_proj = self.proj_path;
        let method = "unknow";
        let args_str = &format!(
            "{args_base} \
        -executeMethod {method} \
        -projectPath {cocos_proj} \
        -targetPlatform:{plat} \
        {ucmdex_args}",
            args_base = args_base,
            method = method,
            cocos_proj = cocos_proj,
            plat = self.plat,
            ucmdex_args = self.ucmdex_args.join(" ")
        );
        let args: Vec<String> = args_str.split(" ").map(|v| v.to_string()).collect();
        info!("Gen the cocos asset...");
        args
    }
}

impl BinCmd for CocosCreatorV2Proj<'_> {
    fn build_ab(&mut self) -> Vec<String> {
        self.base_cmd()
    }

    fn build_player(&mut self) -> Vec<String> {
        self.base_cmd()
    }
}
