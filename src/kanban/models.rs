use pwhash::bcrypt;
use sqlite::{Row, Statement};

#[derive(PartialEq, Eq, Debug)]
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
        for part in parts.iter() {
            if part.is_empty() {
                return Self {
                    board: None,
                    list: None,
                    task: None
                }
            }
        }
        let mut parts = parts.into_iter();
        let ret = Self {
            board: match parts.next() {
                Some(v) if !v.is_empty() => {
                    if !v.is_empty() {
                        Some(v)
                    } else {
                        None
                    }
                },
                _ => None
            },
            list: match parts.next() {
                Some(v) if !v.is_empty() => {
                    if !v.is_empty() {
                        Some(v)
                    } else {
                        None
                    }
                },
                _ => None
            },
            task: match parts.next() {
                Some(v) if !v.is_empty() => {
                    if !v.is_empty() {
                        Some(v)
                    } else {
                        None
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

#[cfg(test)]
mod tests {
    use super::*;

    mod kanban_path_tests {
        use super::KanbanPath;

        #[test]
        fn chech_valid_kanban_board_path() {
            let path: KanbanPath = "board".into();
            assert_eq!(path, KanbanPath{board: Some("board"), list: None, task: None})
        }

        #[test]
        fn chech_invalid_kanban_board_path() {
            let path: KanbanPath = "board.".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None})
        }

        #[test]
        fn chech_invalid_kanban_board_path_2() {
            let path: KanbanPath = ".board".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None})
        }

        #[test]
        fn chech_valid_kanban_list_path() {
            let path: KanbanPath = "board.list".into();
            assert_eq!(path, KanbanPath{board: Some("board"), list: Some("list"), task: None})
        }

        #[test]
        fn chech_invalid_kanban_list_path() {
            let path: KanbanPath = "board.list.".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None})
        }

        #[test]
        fn chech_invalid_kanban_list_path_2() {
            let path: KanbanPath = ".board.list".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None})
        }

        #[test]
        fn chech_valid_kanban_task_path() {
            let path: KanbanPath = "board.list.task".into();
            assert_eq!(path, KanbanPath{board: Some("board"), list: Some("list"), task: Some("task")})
        }

        #[test]
        fn chech_invalid_kanban_task_path() {
            let path: KanbanPath = "board.list.task.".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None})
        }

        #[test]
        fn chech_invalid_kanban_task_path_2() {
            let path: KanbanPath = ".board.list.task".into();
            assert_eq!(path, KanbanPath{board: None, list: None, task: None})
        }
    }


}