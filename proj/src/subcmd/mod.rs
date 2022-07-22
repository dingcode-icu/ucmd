use std::fmt::{Display, Formatter, self};

mod basecmd;
pub mod build_player;
pub mod gen_conf;
pub mod build_ab;
pub mod players;




#[derive(Debug, Copy, Clone)]
pub enum BuildType {
    Android,
    Ios,
    Ab,
}

impl Display for BuildType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            BuildType::Android => { write!(f, "android") }
            BuildType::Ios => { write!(f, "ios") }
            BuildType::Ab => { write!(f, "ab") }
        }
    }
}
