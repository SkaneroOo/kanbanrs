use crate::kanban::models::User;

pub fn repl(user: User) {
    println!("{0} -> {1}", user.idx, user.username);
    // TODO create repl
}