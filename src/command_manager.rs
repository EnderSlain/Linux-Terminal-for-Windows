use crate::commands::{self, Command};
use std::collections::HashMap;

pub struct CommandManager {
    commands: HashMap<&'static str, Box<dyn Command>>,
}

impl CommandManager {
    pub fn new() -> Self {
        let mut commands: HashMap<&'static str, Box<dyn Command>> = HashMap::new();
        commands.insert("clear", Box::new(commands::clear::Clear));
        commands.insert("exit", Box::new(commands::exit::Exit));
        commands.insert("neofetch", Box::new(commands::neofetch::Neofetch));
        commands.insert("fastfetch", Box::new(commands::neofetch::Neofetch));
        commands.insert("ls", Box::new(commands::ls::Ls));
        commands.insert("cd",Box::new(commands::cd::Cd));   

        CommandManager { commands }

    }

    pub fn execute(&self, input: &str) {
        let mut parts = input.trim().split_whitespace();
        if let Some(command_name) = parts.next() {
            let args: Vec<&str> = parts.collect();
            if let Some(command) = self.commands.get(command_name) {
                command.execute(&args);
            } else {
                println!("Command not found: {}", command_name);
            }
        }
    }
}