use sqlite::{State, Statement};

use crate::{kanban::{models::{KanbanPath, User}, db::get_db}, get_statement};

pub fn new(parameters: Vec<&str>, user: &User) {
    if parameters.len() < 2 {
        println!("Provide kanban identifier");
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap().to_owned().into();
    let db = get_db();
    match ident {
        KanbanPath{board: Some(b), list: None, task: None} => {

            let query = "SELECT title FROM boards WHERE idx IN (SELECT board FROM members WHERE user = ?);";
            let mut statement = get_statement!(db, query, user.idx);
            while let Ok(State::Row) = statement.next() {
                if statement.read::<String, _>(0).unwrap().eq_ignore_ascii_case(b) {
                    println!("You're already member of board `{b}`");
                    return
                }
            }
            
            let query = "INSERT INTO boards (owner, title) VALUES (?, ?) RETURNING idx;";
            let mut statement = get_statement!(db, query, user.idx, b);
            let mut idx = -1;
            if let Ok(State::Row) = statement.next() {
                idx = statement.read::<i64, _>(0).unwrap();
            }
            if idx == -1 {
                println!("Something went wrong while creating board");
                return
            }

            let query = "INSERT INTO members (board, user) VALUES (?, ?);";
            let mut statement = get_statement!(db, query, idx, user.idx);
            match statement.next() {
                Ok(State::Done) => {
                    println!("Successfully created new kanban board")
                }
                Err(e) => {
                    println!("Something went wrong while associating board with member");
                    println!("{e}")
                }
                _ => unreachable!("How did it even happen?")
            }

        }
        KanbanPath{board: Some(b), list: Some(l), task: None} => {
            println!("Create list {l} in board {b}")
        }
        KanbanPath{board: Some(b), list: Some(l), task: Some(t)} => {
            println!("Create task {t} in list {l} in board {b}")
        }
        _ => println!("Invalid identifier format")
    }
}