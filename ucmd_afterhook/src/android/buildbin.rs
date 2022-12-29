use async_trait::async_trait;
use fs_extra::{TransitProcess, dir, copy_items_with_progress};
use zip::write::FileOptions;
use zip_extensions::zip_create_from_directory_with_options;

use std::{
    collections::HashMap,
    path::{PathBuf}, env::{temp_dir, self}, fs,
};
use crate::core::{NaAction, BuildBinHook, run_sh};
 
pub struct AndroidNa{}

#[async_trait]
impl NaAction for AndroidNa{ 
    async fn build_cmd(b: &BuildBinHook, arg_map:HashMap<String, String>) {
        //isdev:
        let isdev = match arg_map.get("isDev") {
            Some(v) => v.as_str() == "true",
            None => false,
        };
        //o_type:
        //0:default unity所有导出
        //1:unityLibrary gradle导出
        let def = &String::from("0");
        let o_type = arg_map
            .get("androidPackType")
            .or(Some(def)) 
            .unwrap()
            .as_str();

        //step1: pack output by type
        let f_name = format!("{}_{}.zip", b.plat, b.ver).to_string();
        let build_p = b.proj_path.join(format!(".ucmd_build/{}", b.plat));
        let build_zip = b.proj_path.join(format!(".ucmd_build/{f}", f = f_name));

        match o_type {
            "1" => {
                //build gradle
                let and_path = b.proj_path.join(".ucmd_build/android");
                println!("gradle <unitylibrary> module for pack...");
                if init_gradle(&and_path).await {
                    if !build_gradle(&and_path, isdev).await {
                        println!("[afterbinbuild] gradle build failed!");
                        std::process::exit(1)
                    }
                } else {
                    println!("[afterbinbuild] gradle init failed!");
                    std::process::exit(1)
                };
                println!("pack <unitylibrary> module...");
                AndroidNa::pack_build(&build_p, &build_zip, o_type)
            }
            _ => { 
                println!("pack all module...");
                AndroidNa::pack_build(&build_p, &build_zip, o_type)
            }
        };


        fn pack_build(){

        }
    }

    fn pack_build(fom_dr: &PathBuf,  tar_zip: &PathBuf, p_type: &str) {
        match p_type {
            "1" => {
                pack_gradle_gen(&fom_dr, &tar_zip);
            }
            _ => {
                pack_all_gen(&fom_dr, &tar_zip);
            }
        }
    }
}

pub async fn init_gradle(path: &PathBuf) -> bool {
    let args = vec![
        String::from("wrapper"),
        String::from("-p"),
        path.display().to_string(),
    ];
    let (issuc, _) = run_sh(&env::var("gradle").expect("[hook]not found `gradle` in env executable!"), &args).await;
    issuc
}

pub async fn build_gradle(path: &PathBuf, isdev: bool) -> bool {
    let arg_isdev = format!(
        ":unityLibrary:{}",
        if isdev {
            "assembleDebug"
        } else {
            "assembleRelease"
        }
    );
    let args = vec![arg_isdev, String::from("-p"), path.display().to_string()];
    let (issuc, _) = run_sh(path.join("gradlew").display().to_string().as_str(), &args).await;
    issuc
}

fn pack_gradle_gen(gen_path: &PathBuf, tar_file: &PathBuf) {
    //select build  resoruces
    let lib_gen = vec![
        "unityLibrary/build/outputs/",
        "unityLibrary/src/main/assets/",
    ];
    let temp_p = temp_dir();
    if temp_p.is_dir() {
        let _ = fs::remove_dir_all(&temp_p);
        let _ = fs::create_dir(&temp_p);
    }
    let mut from_paths: Vec<PathBuf> = Vec::new();
    let handle =
        |_: TransitProcess| fs_extra::dir::TransitProcessResult::ContinueOrAbort;
    for _s in lib_gen {
        let pack_t = gen_path.join(_s);
        from_paths.push(pack_t);
    }
    let options = dir::CopyOptions::new(); //
    let r = copy_items_with_progress(&from_paths, temp_p.as_os_str(), &options, handle);
    if r.is_err() {
        println!("--->>copy error is \n{}", r.err().unwrap().to_string());
        panic!("copy select gradle output failed!");
    }

    let r = zip_create_from_directory_with_options(
        &tar_file,
        &temp_p,
        FileOptions::default().compression_method(zip::CompressionMethod::Bzip2),
    );
    if r.is_err() {
        panic!("zip select gradle output failed!");
    }
}

fn pack_all_gen(gen_path: &PathBuf, tar_file: &PathBuf) {
    println!("{} gen path is -->?", gen_path.display());
    zip_create_from_directory_with_options(
        &tar_file,
        &gen_path,
        FileOptions::default().compression_method(zip::CompressionMethod::Bzip2),
    )
    .expect(format!("zip build target raise error!dir={}, tarfile={}", &tar_file.display(), &gen_path.display()).as_str());
}
