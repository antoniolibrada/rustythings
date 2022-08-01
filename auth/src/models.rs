use postgres::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

impl User {
    pub fn from_row(row: &Row) -> User {
        User {
            id: (row.get("id")),
            name: row.get("name"),
            email: row.get("email"),
        }
    }
}
