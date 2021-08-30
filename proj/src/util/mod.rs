use chrono::{DateTime, Utc};
use chrono::format::{StrftimeItems, DelayedFormat};

pub mod filesys;
pub mod shcmd;
pub mod gen_ios;

pub fn get_curtime_str<'a>() -> String {
    let now: DateTime<Utc> = Utc::now();
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
    return now.format("%a %b %e %T %Y").to_string();
}