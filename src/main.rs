mod command_manager;
mod commands;

use crate::command_manager::CommandManager;
use std::io::{self, Write};
use whoami;
use colored::*;
use std::env;

fn main() {

    let manager = CommandManager::new();

    loop {

        let username = whoami::username();
        let hostname = whoami::hostname();
        
        let current_dir = env::current_dir().unwrap_or_default();
        let dir_name = current_dir.file_name().unwrap_or_default().to_str().unwrap_or_default();

        let prompt = format!("{}@{}:~/{}~$ ", username.truecolor(97, 255, 0), hostname.truecolor(97, 255, 0), dir_name.truecolor(0, 140, 255));

        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        manager.execute(&input);
    }
}
