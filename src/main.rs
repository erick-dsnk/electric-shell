extern crate colour;

use std::io;
use std::io::Write;
use std::process::{
    Command,
    exit
};
use std::env;
use std::path::Path;

use colour::*;

fn main() {
    loop {
        let path = env::current_dir().unwrap();

        dark_cyan!("{}\n> ", path.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts;

        match cmd {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);

                if let Err(e) = env::set_current_dir(&root) {
                    e_red_ln!("{}", e);
                }
            },
            "exit" => {
                exit(0);
            }
            _cmd => {
                if cfg!(target_os = "windows") {
                    let mut command = Command::new("cmd")
                        .args(&["/C", _cmd])
                        .args(args)
                        .spawn()
                        .unwrap();
                    
                    command.wait().unwrap();
                } else {
                    let mut command = Command::new("sh")
                        .args(&["-c", _cmd])
                        .args(args)
                        .spawn()
                        .unwrap();
                    
                    command.wait().unwrap();
                }
            }
        };
    }
}