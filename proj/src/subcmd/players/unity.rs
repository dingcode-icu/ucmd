use log::info;
use rcmd_core::chrono::Local;
use rcmd_core::yaml_rust::Yaml;
use super::BinCmd;

pub struct UnityProj<'a> {
    proj_path:&'a str,
    config: &'a Yaml, 
    plat: &'a str, 
    build_path: &'a str, 
    ex_cmd: &'a str
}

impl<'a> UnityProj<'a> {
    pub fn new(proj_path:&'a str, config: &'a Yaml, plat: &'a str, build_path:&'a str, ex_cmd: &'a str) ->Self{
        UnityProj{
            proj_path,
            config,
            plat,
            build_path,
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
        //log file
        let log_path = std::env::current_dir().unwrap().join(".ucmd_build");
        let log_f = log_path.join(format!("{}_unity.log",  Local::now().format("%Y_%m%d_%H%M")));
        let args_str = format!("{args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj} \
        -logFile {log_file} \
        {ex_cmd}",
                                args_base = args_base,
                                method = method,
                                unity_proj = unity_proj,
                                log_file = log_f.display().to_string(),
                                ex_cmd = self.ex_cmd
        );
        
        let args:Vec<String> = args_str.split(" ").map(|v|v.to_string()).collect();
        info!("Gen the unity asset...");
        info!("unity output log in {}", log_f.display().to_string());
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

