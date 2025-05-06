use std::fs;
use std::io::Write;
use colored::*;
use terminal_size::{Width, terminal_size};
use super::Command;

pub struct Ls;

impl Command for Ls {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn execute(&self, _args: &[&str]) {
        let width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80
        };

        match fs::read_dir(".") {
            Ok(entries) => {
                let mut entries: Vec<_> = entries
                    .filter_map(Result::ok)
                    .collect();

                entries.sort_by_key(|e| e.file_name());

                let mut current_line_len = 0;

                for entry in entries {
                    let path = entry.path();
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();

                    let display_len = name_str.len() + 1;

                    if current_line_len + display_len > width {
                        println!();
                        current_line_len = 0;
                    }

                    if path.is_dir() {
                        print!("{} ", name_str.blue());
                    } else {
                        print!("{} ", name_str.truecolor(0, 255, 0));
                    }

                    current_line_len += display_len;
                }
                println!();
            }
            Err(e) => {
                eprintln!("ls: error reading directory: {}", e);
            }
        }

        std::io::stdout().flush().unwrap();
    }
}
