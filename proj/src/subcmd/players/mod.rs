pub mod unity;
pub mod cocosv2;


pub trait BinCmd {
    fn build_ab(&self) -> Vec<String>;
    fn build_player(&self) -> Vec<String>;
}
