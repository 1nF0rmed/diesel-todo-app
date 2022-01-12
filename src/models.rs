use serde::{Serialize, Deserialize};

use super::schema::tasks;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
  pub id: String,
  pub title: String,
  pub descp: String,
  pub completed: bool,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
  pub id: &'a str,
  pub title: &'a str,
  pub descp: &'a str,
}
