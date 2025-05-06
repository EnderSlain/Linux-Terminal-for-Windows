mod command_manager;
mod commands;

use crate::command_manager::CommandManager;
use std::io::{self, Write};
use whoami;
use colored::*;


fn main() {

    let manager = CommandManager::new();

    loop {

        let username = whoami::username();
        let hostname = whoami::hostname();

        let prompt = format!("{}@{}:~$ ", username.red(), hostname.red());

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
