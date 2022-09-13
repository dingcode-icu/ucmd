use std::fmt::{Display, Formatter, self};

mod basecmd;
pub mod build_player;
pub mod gen_conf;
pub mod players;

#[derive(Debug, Copy, Clone)]
pub enum BuildType {
    Android,
    Ios,
    Ab,
    UnSupport
}

impl Display for BuildType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            BuildType::Android => { write!(f, "android") }
            BuildType::Ios => { write!(f, "ios") }
            BuildType::Ab => { write!(f, "ab") }
            BuildType::UnSupport => { write!(f, "unsupport") }
        }
    }
}

impl From<String> for BuildType{
    fn from(s: String) -> Self {
        match s {
            s if s == "android" =>BuildType::Android, 
            s if s == "ios" => BuildType::Ios, 
            s if s == "ab" => BuildType::Ab, 
            _ => BuildType::UnSupport
        }
    }
}