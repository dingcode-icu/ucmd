use super::BinCmd;
use crate::subcmd::BuildType;
use log::info;
use rcmd_core::chrono::Local;
use rcmd_core::yaml_rust::Yaml;
use std::path::{Path};

pub struct UnityProj<'a> {
    proj_path: &'a str,
    config: &'a Yaml,
    plat: BuildType,
    build_path: &'a str,
    ucmdex_args: &'a mut Vec<String>,
}

impl<'a> UnityProj<'a> {
    pub fn new(
        proj_path: &'a str,
        config: &'a Yaml,
        plat: BuildType,
        build_path: &'a str,
        ucmdex_args: &'a mut Vec<String>,
    ) -> Self {
        UnityProj {
            proj_path,
            config,
            plat,
            build_path,
            ucmdex_args,
        }
    }
}

impl UnityProj<'_> {
    pub fn base_cmd(&mut self) -> Vec<String> {
        let config = self.config;
        let args_base = config["args"].as_str().unwrap();
        let unity_proj = self.proj_path;
        let method = config["method"].as_str().unwrap();
        //log file
        let log_path = Path::new(unity_proj).join(".ucmd_build");
        let log_f = log_path.join(format!("{}_unity.log", Local::now().format("%Y_%m%d_%H%M")));
        self.ucmdex_args[0] = String::from(format!("{} -logFile {}",  self.ucmdex_args[0], log_f.display()));
        let args_str = format!(
            "{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        {ucmdex_args}",
            args_base = args_base,
            method = method,
            unity_proj = unity_proj,
            ucmdex_args = self.ucmdex_args[0].as_str()
        );

        let args: Vec<String> = args_str.split(" ").map(|v| v.to_string()).collect();
        info!("Gen the unity asset...");
        info!("unity output log in {}", log_f.display().to_string());
        args
    }
}

impl BinCmd for UnityProj<'_> {
    fn build_ab(&mut self) -> Vec<String> {
        self.base_cmd()
    }

    fn build_player(&mut self) -> Vec<String> {
        self.base_cmd()
    }
}
