// rbst328 - Implementation of Binary Search Tree in Rust
// Copyright (C) 2025  Maciej Sawka <maciejsawka@gmail.com>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


use rbst328::BSTMap;
use std::io::{self, Write};

fn main() {
    let mut bst = BSTMap::<String, String>::new();
    let mut should_quit = false;

    println!("BSTMap REPL!");

    let mut cmd = String::new();
    while !should_quit {
        cmd.clear();

        print!("$ ");
        io::stdout().flush().expect("Error while flushing stdout");
        if let Err(inner) = io::stdin().read_line(&mut cmd) {
            println!("Error while reading input: {}", inner);
            should_quit = true;
        } else {
            let parts: Vec<&str> = cmd.trim().split(" ").collect();

            if parts.len() < 1 {
                println!("Invalid command");
                continue;
            }

            if parts[0] == "set" {
                if parts.len() < 3 {
                    println!("Error: set requires two arguments, key and value");
                    continue;
                }

                let key = parts[1].to_string();
                let value = parts[2].to_string();
                if let Some(old_val) = bst.insert(key, value) {
                    println!("Ok, old value = {}", old_val);
                } else {
                    println!("Ok");
                };

                continue;
            }

            if parts[0] == "get" {
                if parts.len() < 2 {
                    println!("Error: get requires one argument, the key");
                    continue;
                }

                let key = parts[1].to_string();
                if let Some(value) = bst.get(key) {
                    println!("Ok, value = {}", value);
                } else {
                    println!("Err, key not found");
                };

                continue;
            }

            if parts[0] == "del" {
                if parts.len() < 2 {
                    println!("Error: del requires one argument, the key");
                    continue;
                }

                let key = parts[1].to_string();
                if let Some(value) = bst.remove(key) {
                    println!("Ok, old value = {}", value);
                } else {
                    println!("Err, key not found");
                };

                continue;
            }

            if parts[0] == "clear" {
                bst.clear();

                println!("Ok");

                continue;
            }

            if parts[0] == "exit" || parts[0] == "quit" {
                should_quit = true;
                continue;
            }

            if parts[0] == "help" {
                println!("help - display this message");
                println!("set [key] [value] - insert value, overwriting and returning old value if present");
                println!("get [key] - get value associated with the key");
                println!("del [key] - deletes value at given key");
                println!("clear - clear the tree");
                println!("quit, exit - exits the program");

                continue;
            }

            println!("Error, unknown command");
        }
    }
}
