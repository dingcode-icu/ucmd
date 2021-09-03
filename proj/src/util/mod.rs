use chrono::{DateTime, Utc};

pub mod filesys;
pub mod shcmd;
pub mod gen_ios;

pub fn get_ymdt_timestr<'a>() -> String {
    let now: DateTime<Utc> = Utc::now();
    return now.format("%Y%m%T%d").to_string();
}