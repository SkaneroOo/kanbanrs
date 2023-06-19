use std::io::Write;

use sqlite::{self, State, Row};
use pwhash::bcrypt;
use rpassword::read_password;

use super::utils::get_db;
use super::repl::repl;

pub fn help() {
    println!("CLI kanban written in Rust by Skaner\n");
    println!("Available commands:");
    println!("\thelp - show this message");
    println!("\tlogin - login as existing user and start kanban session");
    println!("\tregister - register new user");
}


pub fn register() {
    let connection = get_db();


    print!("Type your username: ");
    let mut username = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_owned();
    
    let mut passwd;
    let mut passwd2;

    loop {
        print!("Type your password: ");
        std::io::stdout().flush().unwrap();
        passwd = read_password().unwrap();
        
        print!("Confirm your password: ");
        std::io::stdout().flush().unwrap();
        passwd2 = read_password().unwrap();

        if passwd == passwd2 {
            break 
        }
        println!("Passwords doesn't match");
    }

    let query = "SELECT * FROM users WHERE username = ?;";

    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, username.as_str())).unwrap();

    if let Ok(State::Row) = statement.next() {
        println!("User with provided username already exists. Try logging in instead.");
        return;
    }

    let hashed = bcrypt::hash(passwd).unwrap();

    let query = "INSERT INTO users (username, password) VALUES (?, ?);";

    statement = connection.prepare(query).unwrap();
    statement.bind((1, username.as_str())).unwrap();
    statement.bind((2, hashed.as_str())).unwrap();

    if let Ok(State::Done) = statement.next() {
        println!("Successfully registered!")
    }

}

pub fn login() {
    let connection = get_db();

    print!("Type your username: ");
    let mut username = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_owned();

    print!("Type your password: ");
    std::io::stdout().flush().unwrap();
    let passwd = read_password().unwrap();

    let query = "SELECT * FROM users WHERE username = ?;";

    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, username.as_str())).unwrap();

    match statement.next() {
        Ok(State::Done) => {
            println!("There's no user with username `{username}`.");
            return;
        }
        Ok(State::Row) => {}
        Err(e) => panic!("{}", e)
    }
    
    let pass_hash = statement.read::<String, _>("password").unwrap();
    if !bcrypt::verify(passwd, pass_hash.as_str()) {
        println!("Invalid password. Try again.");
        return
    }

    let username = statement.read::<String, _>("username").unwrap();
    let idx = statement.read::<i64, _>("idx").unwrap();

    repl(idx, username);

}