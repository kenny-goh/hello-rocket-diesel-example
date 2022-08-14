use crate::schema::*;
use serde::{Deserialize, Serialize};

/// Example of todo entity for update and query
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub name: String,
}

/// Example of todo entity specifically for create
#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub name: String,
}
