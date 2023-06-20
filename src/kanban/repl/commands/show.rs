use crate::kanban::models::KanbanPath;

pub fn show(parameters: Vec<&str>) {
    if parameters.len() < 2 {
        println!("Provide element identifier");
        return
    }
    let ident: KanbanPath = parameters.get(1).unwrap().to_owned().into();
    ident.print();
}