use sqlite::State;

use crate::{
    kanban::{
        models::{
            KanbanPath, 
            User
        }, 
        db::{
            get_database, 
            Database
        }, 
        format_name
    }, 
    get_statement
};

pub fn describe(parameters: &[&str], user: &User) {
    let db = get_database();
    if parameters.len() < 3 {
        println!("Provide kanban identifier and description");
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap_or_else(|| unreachable!()).to_owned().into();
    let description = parameters
                              .to_vec()
                              .splice(2.., vec![])
                              .collect::<Vec<&str>>()
                              .join(" ")
                              .replace("\\n", "\n");
    match ident {
        KanbanPath{board: Some(b), list: None, task: None} => {
            describe_board(&db, b, user, &description);
        }
        KanbanPath{board: Some(b), list: Some(l), task: None} => {
            describe_list(&db, b, l, user, &description);
        }
        KanbanPath{board: Some(b), list: Some(l), task: Some(t)} => {
            describe_task(&db, b, l, t, user, &description);
        }
        _ => println!("Invalid identifier format")
    }
}

fn describe_board(db: &Database, b: &str, user: &User, description: &str) {
    let Some(board) =  db.get_user_board_named(b, user.idx) else {
        println!("You're not a member of board `{b}`");
        return
    };
    let query = "UPDATE boards SET description = ? WHERE idx = ?";
    let mut statement = get_statement!(db, query, description, board.idx);
    if !matches!(statement.next(), Ok(State::Done)) {
        println!("Something went wrong while describing board");
        return
    }
    println!("Successfully added description to board {}", board.title);
}

fn describe_list(db: &Database, b: &str, l: &str, user: &User, description: &str) {
    let Some(board) = db.get_user_board_named(b, user.idx) else {
        println!("You're not a member of board `{b}`");
        return
    };
    let Some(list) = db.get_board_list_named(l, board.idx) else {
        println!("Board `{b}` doesn't contain list `{l}`");
        return
    };
    let query = "UPDATE lists SET description = ? WHERE idx = ?";
    let mut statement = get_statement!(db, query, description, list.idx);
    if !matches!(statement.next(), Ok(State::Done)) {
        println!("Something went wrong while describing list");
        return
    }
    println!("Successfully added description to list {}", list.title);
}

fn describe_task(db: &Database, b: &str, l: &str, t: &str, user: &User, description: &str) {
    let Some(board) =  db.get_user_board_named(b, user.idx) else {
        println!("You're not a member of board `{b}`");
        return
    };
    let Some(list) = db.get_board_list_named(l, board.idx) else {
        println!("Board `{b}` doesn't contain list `{l}`");
        return
    };
    let Some(task) = db.get_list_task_named(t, list.idx) else {
        println!("list `{l}` doesn't contain task `{t}`");
        return
    };
    let query = "UPDATE tasks SET description = ? WHERE idx = ?";
    let mut statement = get_statement!(db, query, description, task.idx);
    if !matches!(statement.next(), Ok(State::Done)) {
        println!("Something went wrong while describing task");
        return
    }
    println!("Successfully added description to task {}", task.title);
}