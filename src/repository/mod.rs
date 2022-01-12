pub mod diesel_repository;

use crate::models::Task;
use crate::error::Result;

pub trait TaskRepository {
  fn create_task(&self, title: String, decription: String) -> Result<Task>;
  fn complete_task(&self, id: String) -> Result<Task>;
}
