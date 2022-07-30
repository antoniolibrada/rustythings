extern crate rusqlite;
extern crate serde;

use crate::model::Todo;

use rusqlite::{params, Connection, Error};

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
        return DataConect { conn };
    }

    fn last_inserted(&self) -> Result<u32, Error> {
        let mut stmt = self.conn.prepare("SELECT last_insert_rowid()").unwrap();
        stmt.query_row([], |row| row.get(0))
    }

    pub fn get(&self, id: &u32) -> Result<Todo, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed from todos where id = (?1)")
            .unwrap();

        stmt.query_row(params![&id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })
    }

    pub fn add(&self, title: &str) -> Result<Todo, Error> {
        let res = self
            .conn
            .execute("INSERT INTO todos (title) values (?1)", params![&title]);
        match res {
            Ok(_) => {
                let last = self.last_inserted().unwrap();
                self.get(&last)
            }
            Err(err) => Err(err),
        }
    }

    pub fn list(&self) -> Result<Vec<Todo>, Error> {
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
            Err(err) => Err(err),
        }
    }

    pub fn update(&self, title: &str, completed: &bool, id: &u32) -> Result<Todo, Error> {
        let mut stmt = self
            .conn
            .prepare("UPDATE todos SET title = (?1), completed = (?2) where id = (?3)")
            .unwrap();

        stmt.execute(params![&title, &completed, &id])?;
        self.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_test() -> DataConect {
        let data = DataConect::new(":memory:");
        data.conn
            .execute("INSERT INTO todos (title) VALUES ('TEST')", [])
            .unwrap();
        data.conn
            .execute("INSERT INTO todos (title) VALUES ('TEST1')", [])
            .unwrap();
        data
    }

    #[test]
    fn it_inserts_data() {
        let data = prepare_test();
        let result = data.list();
        assert_eq!(result.unwrap().len(), 2);
        let inserted = data.add(&"Hello".to_string()).unwrap();
        assert_eq!(inserted.title, "Hello");
        let result = data.list();
        assert_eq!(result.unwrap().len(), 3)
    }

    #[test]
    fn it_gets_one() {
        let data = prepare_test();
        let result = data.get(&1).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.title, "TEST")
    }

    #[test]
    fn it_updates() {
        let data = prepare_test();
        let result = data.update("Title changed", &false, &1).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.title, "Title changed");
        assert_eq!(result.completed, false);
    }

    #[test]
    fn it_lists() {
        let data = prepare_test();
        let result = data.list().unwrap();
        assert_eq!(result.len(), 2);
    }
}
