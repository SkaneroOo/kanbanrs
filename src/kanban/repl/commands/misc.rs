pub fn help() {
    println!("Available commands:");
    println!("\tadd [user] [board] => add new user to board\n");
    println!("\tdescribe [board|list|task] [description] => [");
    println!("\t\t[board] [description] => set board description");
    println!("\t\t[board].[list] [description] => set list description");
    println!("\t\t[board].[list].[task] [description] => set task description");
    println!("\t]\n");
    println!("\texit => exit current session\n");
    println!("\thelp => show this message\n");
    println!("\tnew [board|list|task] => [");
    println!("\t\t[board] => create new board");
    println!("\t\t[board].[list] => create new list in selected board");
    println!("\t\t[board].[list].[task] => create new task in selected list");
    println!("\t]\n");
    println!("\tremove [user] [board] => remove user from board\n");
    println!("\tshow => display your boards");
    println!("\tshow [board|list|task] => [");
    println!("\t\t[board] => display selected board");
    println!("\t\t[board].[list] => display selected list");
    println!("\t\t[board].[list].[task] => display selected task");
    println!("\t]\n");
}