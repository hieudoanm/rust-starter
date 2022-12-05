use super::database::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Task, NewTask};
use serde_json::Value;
use crate::models::TaskData;

#[get("/tasks", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let tasks = Task::get_all_tasks(&conn);
    Json(json!({
        "status": 200,
        "result": tasks,
    }))
}

#[post("/tasks", format = "application/json", data = "<new_task>")]
pub fn new_task(conn: DbConn, new_task: Json<NewTask>) -> Json<Value> {
    Json(json!({
        "status": Task::insert_task(new_task.into_inner(), &conn),
        "result": Task::get_all_tasks(&conn).first(),
    }))
}

#[post("/task", format = "application/json", data = "<task_data>")]
pub fn find_task(conn: DbConn, task_data: Json<TaskData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Task::get_task_by_id(task_data.into_inner(), &conn),
    }))
}
