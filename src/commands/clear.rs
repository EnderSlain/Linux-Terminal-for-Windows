use std::io::Write;
use super::Command;

pub struct Clear;

impl Command for Clear {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn execute(&self, _args: &[&str]) {
        print!("\x1B[3J\x1B[2J\x1B[H");

        std::io::stdout().flush().unwrap();
    }
}