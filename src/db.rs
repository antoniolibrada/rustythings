extern crate rusqlite;
extern crate serde;

use crate::errors::{AppError, AppErrorType};
use crate::model::Todo;

use rusqlite::{params, Connection};

pub struct DataConect {
    pub conn: Connection,
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

    pub fn add(&self, title: &str) -> Result<usize, AppError> {
        let res = self
            .conn
            .execute("INSERT INTO todos (title) values (?1)", params![&title]);
        match res {
            Ok(inserted) => Ok(inserted),
            Err(err) => Err(AppError {
                message: Some("Error inserting item".to_string()),
                cause: Some(err.to_string()),
                error_type: AppErrorType::DbError,
            }),
        }
    }

    pub fn list(&self) -> Result<Vec<Todo>, AppError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed FROM todos")
            .unwrap();

        let todos_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        });

        match todos_iter {
            Ok(iter) => Ok(iter.map(|todo| todo.unwrap()).collect::<Vec<Todo>>()),
            Err(_) => Err(AppError {
                message: Some("Error listing items".to_string()),
                cause: Some("Unkown".to_string()),
                error_type: AppErrorType::DbError,
            }),
        }
    }

    pub fn update(&self, title: &str, completed: &bool, id: &u32) -> Result<usize, AppError> {
        println!("{}, {}, {}", title, completed, id);
        let mut stmt = self
            .conn
            .prepare("UPDATE todos SET title = (?1), completed = (?2) where id = (?3)")
            .unwrap();

        let res = stmt.execute(params![&title, &completed, &id]);
        match res {
            Ok(iter) => Ok(iter),
            Err(_) => Err(AppError {
                message: Some("Error listing items".to_string()),
                cause: Some("Unkown".to_string()),
                error_type: AppErrorType::DbError,
            }),
        }
    }
}
