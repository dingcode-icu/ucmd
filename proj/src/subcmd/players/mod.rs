pub mod unity;
pub mod cocosv2;


pub trait BinCmd {
    fn build_ab(&mut self) -> Vec<String>;
    fn build_player(&mut self) -> Vec<String>;
}
