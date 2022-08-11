use rcmd_core::{Ex::yaml_rust::Yaml, Log::info};
use crate::subcmd::BuildType;
use super::BinCmd;

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
        let ucmdex_args = config["ex_args"].as_str().unwrap();
        let platform_arg =  format!(" -_targetPlatform:{}", &self.plat);
        let args_str = format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        {ex_cmd}",
                                args_base = args_base,
                                method = method,
                                unity_proj = unity_proj,
                                ex_cmd = ucmdex_args.to_string() + &platform_arg
        );
        let args:Vec<String> = args_str.split(" ").map(|v|v.to_string()).collect();
        info!("Gen the unity asset...");
        info!("Full unity command is \n{}", &args.join(" "));
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

