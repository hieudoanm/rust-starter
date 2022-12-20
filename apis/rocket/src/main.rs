#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::http::RawStr;
use rocket::Request;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::string::String;

diesel::table! {
	List (id) {
			id -> Text,
			title -> Text,
			description -> Text,
			createdAt -> Timestamp,
			updatedAt -> Timestamp,
			userId -> Text,
	}
}

diesel::table! {
	Task (id) {
			id -> Text,
			title -> Text,
			description -> Text,
			completed -> Bool,
			createdAt -> Timestamp,
			updatedAt -> Timestamp,
			listId -> Text,
	}
}

diesel::table! {
	User (id) {
			id -> Text,
			username -> Text,
			password -> Text,
			createdAt -> Timestamp,
			updatedAt -> Timestamp,
	}
}

diesel::joinable!(List -> User (userId));
diesel::joinable!(Task -> List (listId));

#[derive(Deserialize, Serialize)]
struct HealthResponse {
    status: String,
}

#[get("/")]
fn index() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[get("/health")]
fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[derive(Deserialize, Serialize)]
struct TaskResponse {
    id: String,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Deserialize)]
struct TaskRequest {
    title: String,
    description: String,
    completed: bool,
}

#[get("/tasks")]
fn get_tasks() -> &'static str {
    "tasks"
}

#[post("/tasks", data = "<new_task>")]
fn create_task(new_task: Json<TaskRequest>) -> &'static str {
    "tasks"
}

#[get("/tasks/<task_id>")]
fn get_task(task_id: &RawStr) -> String {
    task_id.to_string()
}

#[put("/tasks/<task_id>")]
fn update_task(task_id: &RawStr) -> String {
    task_id.to_string()
}

#[delete("/tasks/<task_id>")]
fn delete_task(task_id: &RawStr) -> String {
    task_id.to_string()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/",
            routes![
                index,
                health,
                get_tasks,
                create_task,
                get_task,
                update_task,
                delete_task
            ],
        )
        .launch();
}
