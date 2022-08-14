use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="todos"]
pub struct NewTodo<'x> {
    pub name: &'x str,
}