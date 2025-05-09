pub mod clear;
pub mod exit;
pub mod neofetch;
pub mod ls;
pub mod cd;

pub trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[&str]);
}