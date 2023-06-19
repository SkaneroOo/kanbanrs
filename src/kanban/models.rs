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