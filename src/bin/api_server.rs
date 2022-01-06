extern crate diesel_todo_app;
extern crate diesel;

use self::diesel_todo_app::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
  use diesel_todo_app::schema::tasks::dsl::*;

  let connection = establish_connection();
  let results = tasks.filter(completed.eq(false))
  .limit(5)
  .load::<Task>(&connection)
  .expect("Error loading tasks");

  println!("Displaying {} tasks", results.len());
  for task in results {
    println!("{}", task.title);
  }
  
}