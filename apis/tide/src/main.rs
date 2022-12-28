use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize, Serialize)]
struct Health {
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct List {
    id: String,
    title: String,
    description: String,
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: String,
    title: String,
    description: String,
    completed: bool,
    list_id: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/public/*").serve_dir("public/")?;
    app.at("/health").get(get_health);
    app.at("/users").get(get_users);
    app.at("/users/:id").get(get_user);
    app.at("/lists").get(get_lists);
    app.at("/lists/:id").get(get_list);
    app.at("/tasks").get(get_tasks);
    app.at("/tasks/:id").get(get_task);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_health(mut _req: Request<()>) -> tide::Result {
    Ok(json!(Health {
        status: "healthy".to_string()
    })
    .into())
}

async fn get_users(mut _req: Request<()>) -> tide::Result {
    Ok(json!([]).into())
}

async fn get_user(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap_or("");
    Ok(json!(User {
        id: id.to_string(),
        username: "".to_string(),
        password: "".to_string()
    })
    .into())
}

async fn get_lists(mut _req: Request<()>) -> tide::Result {
    Ok(json!([]).into())
}

async fn get_list(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap_or("");
    Ok(json!(List {
        id: id.to_string(),
        title: "".to_string(),
        description: "".to_string(),
        user_id: "".to_string()
    })
    .into())
}

async fn get_tasks(mut _req: Request<()>) -> tide::Result {
    Ok(json!([]).into())
}

async fn get_task(req: Request<()>) -> tide::Result {
    let id = req.param("id").unwrap_or("");
    Ok(json!(Task {
        id: id.to_string(),
        title: "".to_string(),
        description: "".to_string(),
        completed: false,
        list_id: "".to_string()
    })
    .into())
}
