extern crate rusqlite;
extern crate serde;
use crate::model::Todo;
use rusqlite::Connection;

pub struct DataConect {
    pub conn: Connection,
}

pub enum AppErrorType {
    DbError,
    NotFoundError,
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
            .prepare("SELECT id, title, completed FROM todos")
            .unwrap();
        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    completed: row.get(2).unwrap(),
                })
            })
            .unwrap();

        let a: Vec<_> = todos.map(|res| res.unwrap()).collect();

        return a;
    }
}
