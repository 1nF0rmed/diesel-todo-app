#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod error;
pub mod repository;
pub mod routes;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
// use models::Task;
use std::env;

// use crate::models::NewTask;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// pub fn create_post<'a>(conn: &PgConnection, title: &'a str, descp: &'a str) -> Task {
//     use schema::tasks;

//     let new_task = NewTask {
//         title: title,
//         descp: descp,
//     };

//     diesel::insert_into(tasks::table)
//         .values(&new_task)
//         .get_result(conn)
//         .expect("Error saving new task")
// }
