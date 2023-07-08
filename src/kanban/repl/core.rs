use crate::kanban::{
    models::User, 
    utils::prompt
};
use crate::kanban::repl::commands::{
    help, 
    show, 
    new,
    add
};

pub fn repl(user: &User) {
    loop {
        prompt(">>> ");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("cannot read user input");
        let tokens = command.trim().split(' ').collect::<Vec<&str>>();
        match tokens[0].to_uppercase().as_str() {
            "HELP" => help(),
            "SHOW" => show(&tokens, user),
            "NEW" => new(&tokens, user),
            "ADD" => add(&tokens, user),
            "EXIT" => return,
            _ => {
                println!("ERROR\nCommand {0} is not implemented", tokens[0]);
            }
        }
    }
}