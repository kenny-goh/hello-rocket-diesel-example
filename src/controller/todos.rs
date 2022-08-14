use crate::error::AppError;
use crate::{db, Database, NewTodo, Todo};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

/// Example of create endpoint
#[post("/", data = "<todo>")]
pub async fn insert(database: Database, todo: Json<NewTodo>) -> Result<&'static str, AppError> {
    db::todos::create(&database, todo.0).await
}

/// Example of update endpoint
#[put("/", data = "<todo>")]
pub async fn update(database: Database, todo: Json<Todo>) -> Result<&'static str, AppError> {
    db::todos::update(&database, todo.0).await
}

/// Example of delete endpoint
#[delete("/<id>")]
pub async fn delete(database: Database, id: i32) -> Result<&'static str, AppError> {
    db::todos::delete(&database, id).await
}

/// Example of find endpoint
#[get("/<id>")]
pub async fn find(database: Database, id: i32) -> Result<Json<Todo>, AppError> {
    let resp = db::todos::find(&database, id).await;
    match resp {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(error),
    }
}

/// Example of list endpoint
#[get("/list")]
pub async fn list(database: Database) -> Result<Json<Vec<Todo>>, AppError> {
    let resp = db::todos::list(&database).await;
    match resp {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(error),
    }
}

/// Example of query endpoint
#[get("/query?<text>")]
pub async fn query(database: Database, text: String) -> Result<Json<Vec<Todo>>, AppError> {
    let resp = db::todos::query(&database, text).await;
    match resp {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(error),
    }
}
