use std::{collections::HashMap, error::Error, path::{PathBuf, Path}};

use async_trait::async_trait;
use reqwest::header;
use tokio::process::Command;

#[derive(Default)]
pub struct BuildBinHook {
    pub proj_path: PathBuf,
    pub plat: String,
    pub ver: String,
    pub build_params: String,
    pub log_file: String,
}

impl BuildBinHook {
    pub fn up_to_svr(&self, build_zip:&PathBuf) -> Option<String> {
        let f_name = build_zip.file_name().unwrap();
        //copy *.zip and logfile
        let svr_path = Path::new("/Users/dwb/Desktop/ccmd-server/shizi_dev/bb_sports");
        println!("svr path is dir->{}", svr_path.is_dir());
        let log_tar = svr_path.join(Path::new(&self.log_file).file_name().unwrap());
        println!(
            "logfile is {}, tar file is {}",
            &self.log_file,
            log_tar.display()
        );
        if svr_path.is_dir() {
            std::fs::copy(build_zip, svr_path.join(&f_name))
                .expect("Copy zip file to server failed!");
            std::fs::copy(
                &self.log_file,
                svr_path.join(Path::new(&self.log_file).file_name().unwrap()),
            )
            .unwrap();
            Some(format!("{:?}", &f_name))
        } else {
            println!("Copy zip target path is not exist!");
            None
        }
    }

    pub async fn ding_notice(
        zip_fname: &str,
        build_info: &BuildBinHook,
    ) -> Result<(), Box<dyn Error>> {
        let mut data: HashMap<&str, &str> = HashMap::new();
        data.insert("msgtype", "markdown");

        let mut h = header::HeaderMap::new();
        h.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );

        let cli = reqwest::Client::builder().default_headers(h).build()?;

        let ret = cli.post("https://oapi.dingtalk.com/robot/send?access_token=dd78803e8998c0ae626a501d5858fee66f8d168ad064259bcabcdd2fe5a5f3be")
        .json(&serde_json::json!({
            "msgtype":"markdown", 
            "markdown":{
                "title":"---:3 bb-sports ci ---",
                "text":format!("
### 百分运动unity bb-sports ci

--- 

>plat: {plat}

>version:{ver}
 
>static url: {url}

>build_params: {params}

>log_file: {logfile}
                ", url = format!("http://static.bbclient.icu:8081/bb_sports/{}", zip_fname), ver = build_info.ver, plat = build_info.plat, params = build_info.build_params,
                   logfile= format!("http://static.bbclient.icu:8081/bb_sports/{}", Path::new(&build_info.log_file).file_name().unwrap().to_str().unwrap()))
            }
        }))
        .send()
        .await?
        .text().await?;
        print!("resp txt is {}", ret);
        Ok(())
    }

    pub async fn publish(&self, f: Option<String>) {
        if f.is_some() {
            let ret = BuildBinHook::ding_notice(f.unwrap().as_str(), &self).await;
            if ret.is_err() {
                println!("failed in robot!");
            }
            return;
        }
        panic!("run failed!")
    }
}

#[async_trait]
pub trait NaAction {
    async fn build_cmd(
        b: &BuildBinHook,
        arg_map: HashMap<String, String>
    );
    
    fn pack_build(from_dir: &PathBuf, to_file: &PathBuf, p_type: &str);
}

pub async fn run_sh(cmd: &str, args: &Vec<String>) -> (bool, String) {
    let child = Command::new(cmd).args(args).output().await;
    println!("run sh command ->{} {:?}", cmd, args);
    match child {
        Ok(c) => {
            let ret = String::from_utf8_lossy(&c.stdout).into_owned();
            if c.status.success() {
                return (true, ret);
            }
            println!("{}", String::from_utf8_lossy(&c.stdout).into_owned());
            return (false, String::from_utf8_lossy(&c.stderr).into_owned());
        }
        Err(e) => {
            println!("[hook for ucmd]sh raise error \n{}", e.to_string());
            return (false, e.to_string());
        }
    }
}