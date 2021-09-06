use log::{info, error};
use clap::ArgMatches;
use crate::subcmd::basecmd::BaseCmd;
use std::io::Write;

struct GenConf {
    conf_type: String,
    out_file: String
}


const BUILDPLAYER_CONF : &str = "\
#==========require==========
unity_bin : $Unity                         #unity可执行文件 ex:/Applications/Unity/Hub/Editor/2019.4.26f1c1/Unity.app/Contents/MacOS/Unity
unity_proj : $proj root path               #unity工程路径
log_output_path : $/Users/mac/Desktop      #unity日志输路径
args: -quit -batchmode -isRelease:debug    #通用参数(-isRelease不可删除)
#==========require==========


#==========android require==========
android:
  na_path : $原生工程路径
  method : Ucmd.BuildPlayer.PerformBuildAndroid.ExportProjAsset      #v1.0.0 Ucmd-buildplayer
#==========android require==========


#==========ios require==========
ios :
  na_path : $原生工程路径
  method : ZybEditor.PerformBuild.ExportProjAsset
#==========ios require==========
";

impl BaseCmd for GenConf{}

impl GenConf{
    fn new(ctype: String, outf: String) -> Self {
        GenConf{
            conf_type: ctype.to_string(),
            out_file: outf.to_string()
        }
    }

    fn run(&self){
        let _val = String::from("build-player");
        match &self.conf_type{
            _val=>{
                let mut outf = std::fs::File::create(&self.out_file).expect(format!("create config file {} failed!", &self.out_file).as_str());
                outf.write_all(BUILDPLAYER_CONF.as_bytes()).expect(format!("write content to {} failed!", self.out_file).as_str());
                info!("Gen suc!");
            }
        }
    }
}

pub fn handle(subm: &ArgMatches) {
    let conf_support: Vec<&str> = vec!["build-player"];
    let target = subm.value_of("type");
    let outf = subm.value_of("output").unwrap();
    match target {
        None => {}
        Some(v) => {
            if !conf_support.contains(&v) {
                error!("{}", format!("Not support conf type {} yet! Do nothing", v));
                return;
            }
            let cmd = &GenConf::new(String::from(v), String::from(outf));
            cmd.run();
        }
    };
}