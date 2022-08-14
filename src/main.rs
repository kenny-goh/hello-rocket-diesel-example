#[macro_use]
extern crate diesel;
extern crate core;
extern crate dotenv;
extern crate rocket;

use crate::models::todo::{NewTodo, Todo};
use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket_sync_db_pools::database;

pub mod controller;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;

#[database("app_db")]
pub struct Database(diesel::SqliteConnection);

///
#[get("/")]
fn index() -> &'static str {
    "Hello rocket diesel!"
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![index])
        .mount("/api/todos/", routes![controller::todos::insert])
        .mount("/api/todos/", routes![controller::todos::list])
        .mount("/api/todos/", routes![controller::todos::delete])
        .mount("/api/todos/", routes![controller::todos::query])
        .mount("/api/todos/", routes![controller::todos::update])
        .mount("/api/todos/", routes![controller::todos::find])
}
