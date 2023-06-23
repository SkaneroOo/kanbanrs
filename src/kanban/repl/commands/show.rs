use crate::kanban::{models::{KanbanPath, User}, db::get_db, format_name};

pub fn show(parameters: Vec<&str>, user: &User) {
    let db = get_db();
    if parameters.len() < 2 {
        let boards = db.get_user_boards(user.idx);
        if boards.is_empty() {
            println!("You're not a member of any kanban board");
            return
        }
        println!("Your kanban boards:");
        for board in boards {
            println!("{}", format_name(board.title));
        }
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap().to_owned().into();
    match ident {
        KanbanPath{board: Some(b), list: None, task: None} => {
            let board = match db.get_user_board_named(b, user.idx) {
                Some(sb) => sb,
                None => {
                    println!("You're not a member of board `{b}`");
                    return
                }
            };
            let lists = db.get_board_lists(board.idx);
            println!("{0}{1}", format_name(board.title), {
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
                println!("\t{}", format_name(item.title));
            }
        }
        KanbanPath{board: Some(b), list: Some(l), task: None} => {
            println!("Select list {l} from board {b}")
        }
        KanbanPath{board: Some(b), list: Some(l), task: Some(t)} => {
            println!("Select task {t} from list {l} from board {b}")
        }
        _ => println!("Invalid identifier format")
    }
}