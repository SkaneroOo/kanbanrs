use pwhash::bcrypt;
use sqlite::{Row, Statement};

pub struct User {
    pub idx: i64,
    pub username: String,
    password: String
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        User {
            idx: row.read::<i64, _>("idx"),
            username: row.read::<&str, _>("username").to_owned(),
            password: row.read::<&str, _>("password").to_owned(),
        }
    }
}

impl From<Statement<'_>> for User {
    fn from(stmnt: Statement) -> Self {
        User {
            idx: stmnt.read::<i64, _>("idx").unwrap(),
            username: stmnt.read::<String, _>("username").unwrap(),
            password: stmnt.read::<String, _>("password").unwrap(),
        }
    }
}

impl User {
    pub fn verify_password(&self, pass: String) -> bool {
        bcrypt::verify(pass, self.password.as_str())
    }
}

pub struct KanbanPath<'a> {
    pub board: Option<&'a str>,
    pub list: Option<&'a str>,
    pub task: Option<&'a str>,
}

impl<'a> From<&'a str> for KanbanPath<'a> {
    fn from(value: &'a str) -> Self {
        let mut parts = value.split('.');
        Self {
            board: match parts.next() {
                Some(v) => Some(v),
                None => None
            },
            list: match parts.next() {
                Some(v) => Some(v),
                None => None
            },
            task: match parts.next() {
                Some(v) => Some(v),
                None => None
            }
        }
    }
}

impl KanbanPath<'_> {
    pub fn print(&self) {
        if self.board.is_none() {
            println!("None")
        }
        let mut result = String::from(self.board.unwrap());
        if self.list.is_some() {
            result.push('.');
            result.push_str(self.list.unwrap())
        }
        if self.task.is_some() {
            result.push('.');
            result.push_str(self.task.unwrap())
        }
        println!("{result}")
    }
}