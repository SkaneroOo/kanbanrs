use crate::kanban::models::{KanbanPath, User};

pub fn show(parameters: Vec<&str>, _user: &User) {
    if parameters.len() < 2 {
        println!("Provide kanban identifier");
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap().to_owned().into();
    match ident {
        KanbanPath{board: Some(b), list: None, task: None} => {
            println!("Select board {b}")
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