use crate::error::Result;
use crate::models::{NewTask, Task};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use super::TaskRepository;

pub struct DieselTaskRepository {
    pub conn: PgConnection,
}

impl TaskRepository for DieselTaskRepository {
    fn create_task(&self, title: String, description: String) -> Result<Task> {
        use crate::schema::tasks;

        let new_task = NewTask {
            id: &Uuid::new_v4().to_string(),
            title: &title,
            descp: &description,
        };

        Ok(diesel::insert_into(tasks::table)
            .values(&new_task)
            .get_result(&self.conn)?)
    }

    fn complete_task(&self, id: String) -> Result<Task> {
        use crate::schema::tasks;

        Ok(diesel::update(tasks::dsl::tasks.find(id))
            .set(tasks::dsl::completed.eq(true))
            .get_result::<Task>(&self.conn)?)
    }
}
