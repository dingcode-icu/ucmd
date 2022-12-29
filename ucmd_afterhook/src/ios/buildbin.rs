use std::{collections::HashMap, path::PathBuf};
use async_trait::async_trait;
use crate::core::{NaAction, BuildBinHook};
pub struct IOSNa {}

#[async_trait]
impl NaAction for IOSNa {
    async fn build_cmd(b: &BuildBinHook, arg_map:HashMap<String, String>){
        
    }
    fn pack_build(fom_dr: &PathBuf,  tar_zip: &PathBuf, p_type: &str){}
}


