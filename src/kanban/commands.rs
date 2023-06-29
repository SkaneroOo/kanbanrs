use sqlite::{self, State, Row};
use pwhash::bcrypt;
use rpassword::read_password;

use crate::get_statement;
use crate::kanban::db;
use crate::kanban::utils::prompt;

use super::db::get_database;
use super::repl::repl;

pub fn help() {
    println!("CLI kanban written in Rust by Skaner\n");
    println!("Available commands:");
    println!("\thelp - show this message");
    println!("\tlogin - login as existing user and start kanban session");
    println!("\tregister - register new user");
}


pub fn register() {
    let db = get_database();


    prompt("Type your username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("cannot read stdin input");
    username = username.trim().to_owned();
    
    let mut passwd;
    let mut passwd2;

    loop {
        prompt("Type your password: ");
        passwd = read_password().expect("cannot read user password");
        
        prompt("Confirm your password: ");
        passwd2 = read_password().expect("cannot read user password");

        if passwd == passwd2 {
            break 
        }
        println!("Passwords doesn't match");
    }

    if db.get_user(username.as_str()).is_some() {
        println!("User with provided username already exists. Try logging in instead.");
        return;
    }

    let hashed = bcrypt::hash(passwd).expect("cannot hash password");

    let query = "INSERT INTO users (username, password) VALUES (?, ?);";

    let mut statement = get_statement!(db, query, username.as_str(), hashed.as_str());

    if matches!(statement.next(), Ok(State::Done)) {
        println!("Successfully registered!");
    }

}

pub fn login() {
    let db = get_database();

    prompt("Type your username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("cannot read stdin input");
    username = username.trim().to_owned();

    prompt("Type your password: ");
    let passwd = read_password().expect("cannot read user password");

    let Some(user) = db.get_user(username.as_str()) else {
        println!("There's no user with username `{username}`.");
        return;
    };
    
    if !user.verify_password(passwd) {
        println!("Invalid password. Try again.");
        return
    }

    repl(&user);

}