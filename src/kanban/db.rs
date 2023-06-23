use std::error::Error;

use sqlite::{self, State, Row, Connection, Statement};

use super::models::User;

use crate::get_statement;

pub struct Database {
    conn: Connection
}


pub fn get_db() -> Database {
    let connection = match sqlite::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e)
    };

    let query = "
        CREATE TABLE IF NOT EXISTS users (idx INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, password TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS boards (idx INTEGER PRIMARY KEY AUTOINCREMENT, owner INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
        CREATE TABLE IF NOT EXISTS members (idx INTEGER PRIMARY KEY AUTOINCREMENT, board INTEGER NOT NULL, user INTEGER NOT NULL, permissions INTEGER NOT NULL DEFAULT 0);
        CREATE TABLE IF NOT EXISTS lists (idx INTEGER PRIMARY KEY AUTOINCREMENT, board INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
        CREATE TABLE IF NOT EXISTS tasks (idx INTEGER PRIMARY KEY AUTOINCREMENT, list INTEGER NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '');
        CREATE TABLE IF NOT EXISTS comments (idx INTEGER PRIMARY KEY AUTOINCREMENT, task INTEGER DEFAULT NULL, comment INTEGER DEFAULT NULL, message TEXT NOT NULL);
    ";

    if let Err(err) = connection.execute(query) {
        panic!("Cannot prepare db\n{err}");
    };

    Database{
        conn: connection
    }
}

impl Database {
    #[inline]
    pub fn prepare<T: AsRef<str>>(&self, statement: T) -> std::result::Result<sqlite::Statement<'_>, sqlite::Error> {
        self.conn.prepare(statement)
    }

    
    pub fn get_user(&self, username: &str) -> Option<User> {
        let query = "SELECT * FROM users WHERE username = ?;";

        let mut statement = get_statement!(self.conn, query, username);

        match statement.next() {
            Ok(State::Row) => {
                Some(statement.into())
            },
            Ok(State::Done) => None,
            Err(_) => None
        }
    }

}

#[macro_export]
macro_rules! get_statement {
    ($db:expr, $query:expr, $($binds:expr),*) => {
        {
            let mut statement = ($db).prepare($query).unwrap();
            let mut i = 0;
            $(
                i += 1;
                statement.bind((i, $binds)).unwrap();
            )*
            statement
        }
    };
}