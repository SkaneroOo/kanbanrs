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

pub fn add(parameters: &[&str], user: &User) {
    let db = get_database();
    if parameters.len() < 3 {
        println!("Provide username and board name");
        return
    }

    let Some(new_user) = db.get_user(parameters.get(1).unwrap_or_else(|| unreachable!())) else {
        println!("There's no user with provided username");
        return
    };
    let ident: KanbanPath = parameters.get(1).unwrap_or_else(|| unreachable!()).to_owned().into();

    if ident.board.is_none() {
        println!("Provide valid kanban identifier");
        return
    }

    let name = ident.board.unwrap_or_default();

    let Some(board) = db.get_user_board_named(name, user.idx) else {
        println!("You're not a member of board `{name}`");
        return
    };

    if board.owner != user.idx {
        println!("You cannot add members to board {name}");
        return
    }

    let query = "INSERT INTO members (board, user) VALUES (?, ?);";
    let mut statement = get_statement!(db, query, board.idx, new_user.idx);

    match statement.next() {
        Ok(State::Done) => {
            println!("Successfully added new member to board {name}");
        }
        Err(e) => {
            println!("Something went wrong while associating board with member");
            println!("{e}");
        }
        _ => unreachable!("How did it even happen?")
    }
}