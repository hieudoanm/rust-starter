use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::tasks;
use super::schema::tasks::dsl::tasks as all_tasks;
// this is to get tasks from the database
#[derive(Serialize, Queryable)]
pub struct Task {
		pub id: String,
		pub title: String,
		pub description: String,
		pub completed: bool,
}
// decode request data
#[derive(Deserialize)]
pub struct TaskData {
		pub title: String,
		pub description: String,
		pub completed: bool,
}
// this is to insert tasks to database
#[derive(Debug, Queryable, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
		pub id: String,
		pub title: String,
		pub description: String,
		pub completed: bool,
}

impl Task {

	pub fn get_all_tasks(conn: &PgConnection) -> Vec<Task> {
			all_tasks
					.order(tasks::id.desc())
					.load::<Task>(conn)
					.expect("error!")
	}

	pub fn insert_task(task: NewTask, conn: &PgConnection) -> bool {
			diesel::insert_into(tasks::table)
					.values(&task)
					.execute(conn)
					.is_ok()
	}

	pub fn get_task_by_id(task: TaskData, conn: &PgConnection) -> Vec<Task> {
			all_tasks
					.filter(tasks::id.eq(task.id))
					.load::<Task>(conn)
					.expect("error!")
	}
}
