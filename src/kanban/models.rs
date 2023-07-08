use pwhash::bcrypt;
use sqlite::{
    Row, 
    Statement
};

#[derive(PartialEq, Eq, Debug)]
pub struct User {
    pub idx: i64,
    pub username: String,
    password: String
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            idx: row.read::<i64, _>("idx"),
            username: row.read::<&str, _>("username").to_owned(),
            password: row.read::<&str, _>("password").to_owned(),
        }
    }
}

impl TryFrom<Statement<'_>> for User {
    type Error = &'static str;

    fn try_from(stmnt: Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: stmnt.read::<i64, _>("idx").expect("no field `idx` found"),
            username: stmnt.read::<String, _>("username").expect("no field `username` found"),
            password: stmnt.read::<String, _>("password").expect("no field `password` found"),
        })
    }
}

impl User {
    pub fn verify_password(&self, pass: String) -> bool {
        bcrypt::verify(pass, self.password.as_str())
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct Board {
    pub idx: i64,
    pub owner: i64,
    pub title: String,
    pub description: String
}

impl From<Row> for Board {
    fn from(row: Row) -> Self {
        Self {
            idx: row.read::<i64, _>("idx"),
            owner: row.read::<i64, _>("owner"),
            title: row.read::<&str, _>("title").to_owned(),
            description: row.read::<&str, _>("description").to_owned(),
        }
    }
}

impl TryFrom<Statement<'_>> for Board {
    type Error = &'static str;

    fn try_from(stmnt: Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: stmnt.read::<i64, _>("idx").expect("no field `idx` found"),
            owner: stmnt.read::<i64, _>("owner").expect("no field `owner` found"),
            title: stmnt.read::<String, _>("title").expect("no field `title` found"),
            description: stmnt.read::<String, _>("description").expect("no field `description` found"),
        })
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct List {
    pub idx: i64,
    pub board: i64,
    pub title: String,
    pub description: String
}

impl From<Row> for List {
    fn from(row: Row) -> Self {
        Self {
            idx: row.read::<i64, _>("idx"),
            board: row.read::<i64, _>("board"),
            title: row.read::<&str, _>("title").to_owned(),
            description: row.read::<&str, _>("description").to_owned(),
        }
    }
}

impl TryFrom<Statement<'_>> for List {
    type Error = &'static str;

    fn try_from(stmnt: Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: stmnt.read::<i64, _>("idx").expect("no field `idx` found"),
            board: stmnt.read::<i64, _>("board").expect("no field `board` found"),
            title: stmnt.read::<String, _>("title").expect("no field `title` found"),
            description: stmnt.read::<String, _>("description").expect("no field `description` found"),
        })
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct Task {
    pub idx: i64,
    pub list: i64,
    pub title: String,
    pub description: String
}

impl From<Row> for Task {
    fn from(row: Row) -> Self {
        Self {
            idx: row.read::<i64, _>("idx"),
            list: row.read::<i64, _>("list"),
            title: row.read::<&str, _>("title").to_owned(),
            description: row.read::<&str, _>("description").to_owned(),
        }
    }
}

impl TryFrom<Statement<'_>> for Task {
    type Error = &'static str;

    fn try_from(stmnt: Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: stmnt.read::<i64, _>("idx").expect("no field `idx` found"),
            list: stmnt.read::<i64, _>("list").expect("no field `list` found"),
            title: stmnt.read::<String, _>("title").expect("no field `title` found"),
            description: stmnt.read::<String, _>("description").expect("no field `description` found"),
        })
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct Comment {
    idx: i64,
    task: i64,
    comment: Option<i64>,
    message: String
}

impl From<Row> for Comment {
    fn from(row: Row) -> Self {
        Self {
            idx: row.read::<i64, _>("idx"),
            task: row.read::<i64, _>("task"),
            comment: row.read::<Option<i64>, _>("comment"),
            message: row.read::<&str, _>("message").to_owned(),
        }
    }
}

impl TryFrom<Statement<'_>> for Comment {
    type Error = &'static str;

    fn try_from(stmnt: Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            idx: stmnt.read::<i64, _>("idx").expect("no field `idx` found"),
            task: stmnt.read::<i64, _>("task").expect("no field `task` found"),
            comment: stmnt.read::<Option<i64>, _>("comment").expect("no field `comment` found"),
            message: stmnt.read::<String, _>("message").expect("no field `message` found"),
        })
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct KanbanPath<'a> {
    pub board: Option<&'a str>,
    pub list: Option<&'a str>,
    pub task: Option<&'a str>,
}

impl<'a> From<&'a str> for KanbanPath<'a> {
    fn from(value: &'a str) -> Self {
        let parts = value.split('.').collect::<Vec<&str>>();
        if parts.len() > 3 {
            return Self {
                board: None,
                list: None,
                task: None
            }
        }
        for part in &parts {
            if part.is_empty() {
                return Self {
                    board: None,
                    list: None,
                    task: None
                }
            }
            for ch in part.chars() {
                match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {},
                    _ => return Self {
                        board: None,
                        list: None,
                        task: None
                    }
                }
            }
        }
        let mut parts = parts.into_iter();
        let ret = Self {
            board: match parts.next() {
                Some(v) if !v.is_empty() => {
                    if v.is_empty() {
                        None
                    } else {
                        Some(v)
                    }
                },
                _ => None
            },
            list: match parts.next() {
                Some(v) if !v.is_empty() => {
                    if v.is_empty() {
                        None
                    } else {
                        Some(v)
                    }
                },
                _ => None
            },
            task: match parts.next() {
                Some(v) if !v.is_empty() => {
                    if v.is_empty() {
                        None
                    } else {
                        Some(v)
                    }
                },
                _ => None
            }
        };
        if ret.board.is_none() && (ret.list.is_some() || ret.task.is_some()) {
            return KanbanPath{board: None, list: None, task: None}
        }
        if ret.list.is_none() && ret.task.is_some() {
            return KanbanPath{board: None, list: None, task: None}
        }
        ret
    }
}

impl KanbanPath<'_> {
    #[allow(dead_code)]
    pub fn print(&self) {
        if self.board.is_none() {
            println!("None");
        }
        let mut result = String::from(self.board.unwrap_or_else(|| unreachable!()));
        if let Some(list) = self.list {
            result.push('.');
            result.push_str(list);
        }
        if let Some(task) = self.task {
            result.push('.');
            result.push_str(task);
        }
        println!("{result}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod kanban_path_tests {
        use super::KanbanPath;

        #[test]
        fn chech_valid_kanban_board_path() {
            let path: KanbanPath = "board".into();
            assert_eq!(path, KanbanPath{board: Some("board"), list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_board_path() {
            let path: KanbanPath = "board.".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_board_path_2() {
            let path: KanbanPath = ".board".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_board_path_3() {
            let path: KanbanPath = "bóard".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_valid_kanban_list_path() {
            let path: KanbanPath = "1board.list".into();
            assert_eq!(path, KanbanPath{board: Some("1board"), list: Some("list"), task: None});
        }

        #[test]
        fn chech_invalid_kanban_list_path() {
            let path: KanbanPath = "board.list.".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_list_path_2() {
            let path: KanbanPath = ".board.list".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_list_path_3() {
            let path: KanbanPath = "board.liśt".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_valid_kanban_task_path() {
            let path: KanbanPath = "board.list2.task".into();
            assert_eq!(path, KanbanPath{board: Some("board"), list: Some("list2"), task: Some("task")});
        }

        #[test]
        fn chech_invalid_kanban_task_path() {
            let path: KanbanPath = "board.list.task.".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_task_path_2() {
            let path: KanbanPath = ".board.list.task".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }

        #[test]
        fn chech_invalid_kanban_task_path_3() {
            let path: KanbanPath = "board.lis t.task".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None});
        }
    }


}