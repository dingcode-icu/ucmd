use chrono::{DateTime, Utc};
use chrono::format::{StrftimeItems, DelayedFormat};

pub mod filesys;
pub mod shcmd;
pub mod gen_ios;

pub fn get_Ymdt_timestr<'a>() -> String {
    let now: DateTime<Utc> = Utc::now();
    return now.format("%Y%m%T%d").to_string();
}