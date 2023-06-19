use sqlite::{self, State, Row, Connection};


pub fn get_db() -> Connection {
    let connection = match sqlite::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e)
    };

    let query = "
        CREATE TABLE IF NOT EXISTS users (idx INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, password TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS boards (idx INTEGER PRIMARY KEY AUTOINCREMENT, owner INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
        CREATE TABLE IF NOT EXISTS members (idx INTEGER PRIMARY KEY AUTOINCREMENT, board INTEGER NOT NULL, user INTEGER NOT NULL, permissions INTEGER NOT NULL);
        CREATE TABLE IF NOT EXISTS lists (idx INTEGER PRIMARY KEY AUTOINCREMENT, board INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
        CREATE TABLE IF NOT EXISTS tasks (idx INTEGER PRIMARY KEY AUTOINCREMENT, list INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
        CREATE TABLE IF NOT EXISTS comments (idx INTEGER PRIMARY KEY AUTOINCREMENT, task INTEGER DEFAULT NULL, comment INTEGER DEFAULT NULL, message TEXT NOT NULL);
    ";

    if let Err(err) = connection.execute(query) {
        panic!("Cannot prepare db\n{err}");
    };

    connection
}