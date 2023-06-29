use sqlite::{State, Statement};

use crate::{kanban::{models::{KanbanPath, User, Board, List, Task}, db::{get_db, Database}}, get_statement};

pub fn new(parameters: Vec<&str>, user: &User) {
    if parameters.len() < 2 {
        println!("Provide kanban identifier");
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap().to_owned().into();
    let db = get_db();
    match ident {
        KanbanPath{board: Some(b), list: None, task: None} => {
            
            if db.get_user_board_named(b, user.idx).is_some() {
                println!("You're already member of board `{b}`");
                return
            }
            
            let query = "INSERT INTO boards (owner, title) VALUES (?, ?) RETURNING idx;";
            let mut statement = get_statement!(db, query, user.idx, b);
            let mut idx = -1;
            if let Ok(State::Row) = statement.next() {
                idx = statement.read::<i64, _>(0).unwrap_or(-1);
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
            
            let board = match db.get_user_board_named(b, user.idx) {
                None => {
                    println!("You're not a member of board `{b}`");
                    return
                },
                Some(b) => b
            };

            if db.get_board_list_named(l, board.idx).is_some() {
                println!("Board {b} already contains list {l}");
                return
            }
            
            let query = "INSERT INTO lists (board, title) VALUES (?, ?);";
            let mut statement = get_statement!(db, query, board.idx, l);
            if let Err(e) =  statement.next() {
                println!("Something went wrong while creating board\n{e}");
            }
            println!("Successfully created new list")
        }
        KanbanPath{board: Some(b), list: Some(l), task: Some(t)} => {
            
            let board = match db.get_user_board_named(b, user.idx) {
                None => {
                    println!("You're not a member of board `{b}`");
                    return
                },
                Some(sb) => sb
            };
            
            let list = match db.get_board_list_named(l, board.idx) {
                None => {
                    println!("Board `{b}` doesn't contain list `{l}`");
                    return
                },
                Some(sl) => sl
            };

            if db.get_list_task_named(t, list.idx).is_some() {
                println!("List {b} already contains task {l}");
                return
            }
            
            let query = "INSERT INTO tasks (list, title) VALUES (?, ?);";
            let mut statement = get_statement!(db, query, list.idx, t);
            if let Err(e) =  statement.next() {
                println!("Something went wrong while creating task\n{e}");
            }
            println!("Successfully created new kanban task")
        }
        _ => println!("Invalid identifier format")
    }
}