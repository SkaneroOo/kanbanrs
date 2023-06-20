pub fn help() {
    println!("Available commands:");
    println!("\thelp => Show this message");
    println!("\tlist => [");
    println!("\t\tboards => list your kanban boards");
    println!("\t\tmembers [board] => list mambers of selected board");
    println!("\t\tlists [board] => list task lists available in selected board");
    println!("\t\ttasks [board].[list] => list tasks available in selected list");
    println!("\t]");
    println!("\tnew [board|list|task] => [");
    println!("\t\tboard [name] => create new board");
    println!("\t\tlist [board].[name] => create new list in selected board");
    println!("\t\ttask [board].[list].[name] => create new task in selected list");
    println!("\t]");
    println!("\tshow [board|list|task] => [");
    println!("\t\t[board] => display selected board");
    println!("\t\t[board].[list] => display selected list");
    println!("\t\t[board].[list].[task] => display selected task");
    println!("\t]");
}