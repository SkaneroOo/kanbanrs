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

pub fn remove(parameters: &[&str], user: &User) {
    let db = get_database();
    if parameters.len() < 3 {
        println!("Provide username and board name");
        return
    }

    let Some(selected_user) = db.get_user(parameters.get(1).unwrap_or_else(|| unreachable!())) else {
        println!("There's no user with provided username");
        return
    };

    if selected_user.idx == user.idx {
        println!("You cannot remove yourself from board");
        return
    }

    let ident: KanbanPath = parameters.get(2).unwrap_or_else(|| unreachable!()).to_owned().into();

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
        println!("You cannot remove members from board {name}");
        return
    }

    let query = "SELECT * FROM members WHERE board = ? and user = ?";
    let mut statement = get_statement!(db, query, board.idx, selected_user.idx);

    if matches!(statement.next(), Ok(State::Done)) {
        println!("Selected user is not a member of selected board");
        return
    }

    let query = "DELETE FROM members WHERE board = ? and user = ?";
    let mut statement = get_statement!(db, query, board.idx, selected_user.idx);

    match statement.next() {
        Ok(State::Done) => {
            println!("Successfully removed member from board {name}");
        }
        Err(e) => {
            println!("Something went wrong while associating board with member");
            println!("{e}");
        }
        _ => unreachable!("How did it even happen?")
    }
}