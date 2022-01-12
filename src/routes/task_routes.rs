use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    establish_connection,
    models::Task,
    repository::{diesel_repository::DieselTaskRepository, TaskRepository},
};

#[derive(Serialize, Deserialize)]
pub struct TaskRequest {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskCompleteRequest {
    completed: bool,
}

fn default_task_data() -> Vec<Task> {
    Vec::new()
}

#[derive(Serialize, Deserialize)]
pub struct StatusMessage {
    status: String,
    message: String,
    #[serde(default = "default_task_data")]
    data: Vec<Task>,
}

pub fn create_task(task: web::Json<TaskRequest>) -> HttpResponse {
    let task_repo = DieselTaskRepository {
        conn: establish_connection(),
    };

    let result = task_repo.create_task(task.title.clone(), task.description.clone());

    match result {
        Ok(task) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "ok".to_string(),
                message: "".to_string(),
                data: vec![task],
            }),
        Err(_err) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "error".to_string(),
                message: "Unable to add task".to_string(),
                data: default_task_data(),
            }),
    }
}

pub fn complete_task(
    task_id: web::Path<String>,
    task: web::Json<TaskCompleteRequest>,
) -> HttpResponse {
    let task_repo = DieselTaskRepository {
        conn: establish_connection(),
    };

    if !task.completed {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "error".to_string(),
                message: "Cannot update task completion to false".to_string(),
                data: default_task_data(),
            });
    }

    let result = task_repo.complete_task(task_id.to_string());

    match result {
        Ok(task) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "ok".to_string(),
                message: "".to_string(),
                data: vec![task],
            }),
        Err(_err) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "error".to_string(),
                message: "Unable to complete task".to_string(),
                data: default_task_data(),
            }),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{test, web, App};
    use diesel::{query_dsl::methods::FilterDsl};

    #[actix_rt::test]
    async fn test_should_create_task() {
        let data = TaskRequest {
            title: "Adding a new PR".to_string(),
            description: "Update base repo to implement serde ro parse data".to_string(),
        };
        let mut app =
            test::init_service(App::new().route("/tasks/create", web::post().to(create_task)))
                .await;
        let req = test::TestRequest::post()
            .uri("/tasks/create")
            .set_json(data)
            .to_request();
        let resp: StatusMessage = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.status, "ok".to_string());

        use diesel::prelude::*;
        use crate::schema::tasks::dsl::*;
        diesel::delete(FilterDsl::filter(tasks, id.eq(resp.data[0].id.to_string())))
            .execute(&establish_connection())
            .expect("Couldn't teardown for test");
    }
}
