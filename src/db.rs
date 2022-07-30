extern crate rusqlite;
extern crate serde;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub struct DataConect {
    pub conn: Connection,
}

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

pub enum AppErrorType {
    DbError,
    NotFoundError,
}

pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl DataConect {
    pub fn new(db_name: &str) -> DataConect {
        let conn = Connection::open(db_name).unwrap();
        conn.execute(
            "create table if not exists todos (
                id integer primary key,
                title text not null,
                completed boolean DEFAULT false
            )",
            [],
        )
        .unwrap();
        return DataConect { conn: conn };
    }

    pub fn add(&self, title: &str) -> bool {
        self.conn
            .execute("INSERT INTO todos (title) values (?1)", &[title])
            .unwrap();

        return true;
    }

    pub fn list(&self) -> Vec<Todo> {
        let mut stmt = self
            .conn
            .prepare("SELECT title, completed FROM todos")
            .unwrap();
        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    title: row.get(0).unwrap(),
                    completed: row.get(1).unwrap(),
                })
            })
            .unwrap();

        let a: Vec<_> = todos.map(|res| res.unwrap()).collect();

        return a;
    }
}
