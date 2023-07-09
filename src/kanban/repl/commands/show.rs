use crate::kanban::{
    models::{
        KanbanPath, 
        User
    }, 
    db::{
        get_database, 
        Database
    }, 
    format_name
};

pub fn show(parameters: &[&str], user: &User) {
    let db = get_database();
    if parameters.len() < 2 {
        let boards = db.get_user_boards(user.idx);
        if boards.is_empty() {
            println!("You're not a member of any kanban board");
            return
        }
        println!("Your kanban boards:");
        for board in boards {
            println!("{}", format_name(&board.title));
        }
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap_or_else(|| unreachable!()).to_owned().into();
    match ident {
        KanbanPath{board: Some(b), list: None, task: None} => {
            show_board(&db, b, user);
        }
        KanbanPath{board: Some(b), list: Some(l), task: None} => {
            show_list(&db, b, l, user);
        }
        KanbanPath{board: Some(b), list: Some(l), task: Some(t)} => {
            show_task(&db, b, l, t, user);
        }
        _ => println!("Invalid identifier format")
    }
}

fn show_board(db: &Database, b: &str, user: &User) {
    let Some(board) =  db.get_user_board_named(b, user.idx) else {
        println!("You're not a member of board `{b}`");
        return
    };
    let lists = db.get_board_lists(board.idx);
    println!("{0}{1}", format_name(&board.title), {
        if board.description.is_empty() {
            String::new()
        } else {
            format!(" - {}", board.description)
        }
    });
    if lists.is_empty() {
        println!("Selected kanban board is empty.");
        return
    }
    for item in lists {
        println!("\t{}", format_name(&item.title));
    }
}

fn show_list(db: &Database, b: &str, l: &str, user: &User) {
    let Some(board) = db.get_user_board_named(b, user.idx) else {
        println!("You're not a member of board `{b}`");
        return
    };
    let Some(list) = db.get_board_list_named(l, board.idx) else {
        println!("Board `{b}` doesn't contain list `{l}`");
        return
    };
    println!("{0}{1}", format_name(&list.title), {
        if list.description.is_empty() {
            String::new()
        } else {
            format!(" - {}", list.description)
        }
    });
    let tasks = db.get_list_tasks(list.idx);
    if tasks.is_empty() {
        println!("Selected list is empty.");
        return
    }
    for item in tasks {
        println!("\t{}", format_name(&item.title));
    }
}

fn show_task(db: &Database, b: &str, l: &str, t: &str, user: &User) {
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
    println!("{0}{1}", format_name(&task.title), {
        if task.description.is_empty() {
            String::new()
        } else {
            format!(" - {}", task.description)
        }
    });
}