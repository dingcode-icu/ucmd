use crate::subcmd::basecmd::BaseCmd;
use crate::util;
use yaml_rust::Yaml;
use clap::ArgMatches;

const UNITY_CMD: &str = "";
const PACKAGE_CONF: &str = "/Users/mac/data0/public_work/ucmd/proj/src/package.yaml";


struct BuildPlayer {
    build_config: Yaml,
    platform: String,
    isr : bool
}

impl BaseCmd for BuildPlayer {}

impl BuildPlayer {
    fn new(platform: String, isr: bool) -> Self {
        BuildPlayer {
            build_config: BuildPlayer::parse_config(PACKAGE_CONF),
            platform,
            isr
        }
    }
    fn gen_unity_asset(&self) {
        let base = &self.build_config;
        // let plat
        let unity_bin = base["unity_bin"].as_str();
        println!("{:?}", base);
        let cmd = format!("{unity_bin} {args_base} \
        -executeMethod {method} \
        -projectPath {unity_proj}\
        -logfile {logfile}",
                          unity_bin = unity_bin.unwrap(),
                          args_base = base["args"].as_str().unwrap(),
                          method = base["method"].as_str().unwrap(),
                          unity_proj = base["unity_proj"].as_str().unwrap(),
                          logfile = base["log_output_path"].as_str().unwrap().to_string() + util::get_curtime_str().as_str()
        );
    }
    fn run(&self) {
        self.gen_unity_asset();
    }
}


pub fn handle(subm: &ArgMatches) {
    let isr = subm.is_present("release");
    match subm.value_of("plat"){
        None => {}
        Some(v) => {
            let PLAT_SUPPORT : Vec<&str>= vec!["android", "ios"];
            if !PLAT_SUPPORT.contains(&v){
                println!("Not support platform {} yet! Do nothing", v);
                return;
            }
            let cmd = &BuildPlayer::new(v.to_string(), isr);
            cmd.run();
        }
    };
}