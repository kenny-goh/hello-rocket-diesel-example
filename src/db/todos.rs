use crate::error::AppError;
use crate::schema;
use crate::schema::todos::name;
use crate::{Database, NewTodo, Todo};
use diesel::result::Error as DieselError;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;

/// Example of create method in DB
pub async fn create(database: &Database, new_todos: NewTodo) -> Result<&'static str, AppError> {
    use schema::todos;
    let resp: Result<usize, DieselError> = database
        .run(move |c| {
            diesel::insert_into(todos::table)
                .values(new_todos)
                .execute(c)
        })
        .await;
    match resp {
        Ok(_) => Ok(""),
        Err(_) => Err(AppError::DBError(String::from("Failed to create record."))),
    }
}

/// Example of update method in DB
pub async fn update(database: &Database, todo: Todo) -> Result<&'static str, AppError> {
    use schema::todos::dsl::todos;
    let resp: Result<usize, DieselError> = database
        .run(move |c| {
            diesel::update(todos.find(todo.id))
                .set(name.eq(todo.name))
                .execute(c)
        })
        .await;
    match resp {
        Ok(updated) => {
            if updated > 0 {
                Ok("")
            } else {
                Err(AppError::NotFoundError(String::from("Record not found.")))
            }
        }
        Err(_) => Err(AppError::DBError(String::from("Failed to update record."))),
    }
}

/// Example of find method in DB
pub async fn find(database: &Database, id: i32) -> Result<Todo, AppError> {
    use schema::todos::dsl::todos;
    let resp: Result<Todo, DieselError> = database.run(move |c| todos.find(id).first(c)).await;
    match resp {
        Ok(result) => Ok(result),
        Err(_) => Err(AppError::DBError(String::from("Failed to find record."))),
    }
}

/// Example of delete method in DB
pub async fn delete(database: &Database, id: i32) -> Result<&'static str, AppError> {
    use schema::todos::dsl::todo_id;
    use schema::todos::dsl::todos;
    let resp: Result<usize, DieselError> = database
        .run(move |c| diesel::delete(todos.filter(todo_id.eq(id))).execute(c))
        .await;
    match resp {
        Ok(deleted) => {
            if deleted > 0 {
                Ok("")
            } else {
                Err(AppError::NotFoundError(String::from("Record not found.")))
            }
        }
        Err(_) => Err(AppError::DBError(String::from("Failed to delete record."))),
    }
}

/// Example of list method in DB
pub async fn list(database: &Database) -> Result<Vec<Todo>, AppError> {
    use schema::todos::dsl::todos;
    let resp: Result<Vec<Todo>, DieselError> = database.run(|c| todos.load::<Todo>(c)).await;
    match resp {
        Ok(result) => Ok(result),
        Err(_) => Err(AppError::DBError(String::from("Failed to create record."))),
    }
}

/// Example of query method in DB
pub async fn query(database: &Database, str: String) -> Result<Vec<Todo>, AppError> {
    use schema::todos::dsl::todos;
    let resp: Result<Vec<Todo>, DieselError> = database
        .run(move |c| todos.filter(name.like(format!("{}%", str))).load::<Todo>(c))
        .await;
    match resp {
        Ok(result) => Ok(result),
        Err(_) => Err(AppError::DBError(String::from("Failed to create record."))),
    }
}
