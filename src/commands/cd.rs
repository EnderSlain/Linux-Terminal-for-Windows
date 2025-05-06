use std::env;
use std::io::{self, Write};
use super::Command;

pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn execute(&self, args: &[&str]) {
        if args.is_empty() {
            eprintln!("cd: missing argument");
            return;
        }

        let dir = args[0];
        
        if dir == ".." {
            if let Err(e) = env::current_dir() {
                eprintln!("cd: error: {}", e);
                return;
            }

            if let Err(e) = env::set_current_dir("..") {
                eprintln!("cd: error: {}", e);
            }
        } else {

            if let Err(e) = env::set_current_dir(dir) {
                eprintln!("cd: error: {}", e);
            }
        }
    }
}
