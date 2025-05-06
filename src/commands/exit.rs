use std::process;
use super::Command;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn execute(&self, _args: &[&str]) {
        process::exit(0);
    }
}