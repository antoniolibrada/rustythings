use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DTOTodoInput {
    pub title: String,
}
