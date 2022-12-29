mod android;
mod core;
mod ios;

use dotenv::dotenv;
use std::collections::HashMap;
use crate::core::{BuildBinHook, NaAction};

/// 预处理unity + ucmd命令传入的参数
///
/// display_argmap是为了打印展示用
/// >return arg_map
/// 函数返回的是hashmap过的参数
/// 返回值是（arg_map, display_argmap）,前者用于后续操作， 后者精简了内置参
/// 和log参的展示
/// * _targetPlatform
/// * logFile
fn pre_chk_params() -> (HashMap<String, String>, HashMap<String, String>) {
    let args: Vec<String> = std::env::args().collect();
    let args_params: Vec<String> = args.get(1).expect("[hook]argument lenth is empty").split(" -").map(|v| v.to_string()).collect();
    let mut arg_map = HashMap::new();
    let mut display_argmap = HashMap::new();
    for cell in &args_params[1..] {
        let c = cell
            .clone()
            .replace(' ', ":")
            .split_once(":")
            .and_then(|(k, v)| {
                let ret = arg_map.insert(k.to_string(), v.to_string().trim().to_string());
                if !k.to_string().starts_with("_") && !k.to_string().contains("log") {
                    display_argmap.insert(k.to_string(), v.to_string());
                }
                ret
            });
        if c.is_some() {
            println!("key <{}>is override!", c.unwrap());
        }
    }
    //struct with inner params
    return (arg_map, display_argmap);
}

#[tokio::main]
async fn main() {
    //dotenv 
    dotenv().ok();
    //chk `BuildBinHook` arguments
    let (arg_map, display_argmap) = pre_chk_params();
    let cur = std::env::current_exe().unwrap();
    let ver = chrono::Local::now().format("%m%d-%H%M%S");
    let def_log = &String::from("None");
    let proj_path = cur.parent().unwrap().parent().unwrap(); 
    let log_f = arg_map.get("logFile").or(Some(def_log)).unwrap();
    //chk platfom
    let bhook = BuildBinHook {
        proj_path: proj_path.to_path_buf(),
        plat: arg_map.get("_targetPlatform").unwrap().to_string(),
        ver: ver.to_string(),
        build_params: format!("{:?}", display_argmap),
        log_file: log_f.to_string(),
    };
    let ret = arg_map
        .get("_targetPlatform")
        .expect("[hook]not found `_targetPlatform` params")
        .as_str();
    match ret {
        "android" => {
            use android::buildbin::AndroidNa;
            AndroidNa::build_cmd(&bhook, arg_map).await;
        }

        "ios" => {
            println!("test none");
        }

        _ => {
            println!("[hook]{} not support yet!", ret);
        }
    };

    //step2: publish zip file
    // bhook.publish(f).await;
}

#[tokio::test]
async fn test_main() {
    let cur = std::env::current_dir().unwrap();
    println!("cur {}", cur.display());
    let test = cur.join("test").to_path_buf();

    let ver = chrono::Local::now().format("%m%d-%H%M%S");
    println!("cur time is {}", ver);
}
