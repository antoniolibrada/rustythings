use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DTOAddTodoInput {
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct DTOUpdateTodoInput {
    pub title: String,
    pub completed: bool,
}
