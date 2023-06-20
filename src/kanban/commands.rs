use sqlite::{self, State, Row};
use pwhash::bcrypt;
use rpassword::read_password;

use crate::kanban::db;
use crate::kanban::utils::prompt;

use super::db::get_db;
use super::repl::repl;

pub fn help() {
    println!("CLI kanban written in Rust by Skaner\n");
    println!("Available commands:");
    println!("\thelp - show this message");
    println!("\tlogin - login as existing user and start kanban session");
    println!("\tregister - register new user");
}


pub fn register() {
    let db = get_db();


    prompt("Type your username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_owned();
    
    let mut passwd;
    let mut passwd2;

    loop {
        prompt("Type your password: ");
        passwd = read_password().unwrap();
        
        prompt("Confirm your password: ");
        passwd2 = read_password().unwrap();

        if passwd == passwd2 {
            break 
        }
        println!("Passwords doesn't match");
    }

    let query = "SELECT * FROM users WHERE username = ?;";

    let mut statement = db.prepare(query).unwrap();
    statement.bind((1, username.as_str())).unwrap();

    if db.get_user(username.as_str()).is_some() {
        println!("User with provided username already exists. Try logging in instead.");
        return;
    }

    let hashed = bcrypt::hash(passwd).unwrap();

    let query = "INSERT INTO users (username, password) VALUES (?, ?);";

    statement = db.prepare(query).unwrap();
    statement.bind((1, username.as_str())).unwrap();
    statement.bind((2, hashed.as_str())).unwrap();

    if let Ok(State::Done) = statement.next() {
        println!("Successfully registered!")
    }

}

pub fn login() {
    let db = get_db();

    prompt("Type your username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_owned();

    prompt("Type your password: ");
    let passwd = read_password().unwrap();

    let user = db.get_user(username.as_str());

    if user.is_none() {
        println!("There's no user with username `{username}`.");
        return;
    }

    let user = user.unwrap();
    
    if !user.verify_password(passwd) {
        println!("Invalid password. Try again.");
        return
    }

    repl(user);

}