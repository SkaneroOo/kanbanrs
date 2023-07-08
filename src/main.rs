#![allow(unreachable_code, unused_imports)]
use std::{
    env, 
    println
};

mod kanban;
use kanban::{
    help, 
    login, 
    register
};


fn main() {

    let mut args = env::args();

    let name = args.next().unwrap_or_default();
    
    let Some(command) = args.next() else {
        println!("CLI kanban written in Rust by Skaner\n");
        println!("For usage information use {name} help");
        return;
    };

    match command.as_str() {
        "help" => {
            help();
        }
        "register" => {
            register();
        }
        "login" => {
            login();
        }
        _ => {
            println!("Invalid command {command}. For usage instruction use `{name} help`");

        }
    }

}