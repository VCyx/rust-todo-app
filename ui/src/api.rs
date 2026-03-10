use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoModel {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Serialize)]
pub struct UpdateTodo {
    pub title: String,
    pub completed: bool,
}

const API_URL: &str = "http://localhost:3000/todos";

pub async fn list_todos() -> Result<Vec<TodoModel>, String> {
    let res = reqwest::get(API_URL).await.map_err(|e| e.to_string())?;
    res.json::<Vec<TodoModel>>().await.map_err(|e| e.to_string())
}

pub async fn get_todo(id: i32) -> Result<TodoModel, String> {
    let res = reqwest::get(&format!("{}/{}", API_URL, id)).await.map_err(|e| e.to_string())?;
    res.json::<TodoModel>().await.map_err(|e| e.to_string())
}

pub async fn create_todo(title: String) -> Result<TodoModel, String> {
    let client = reqwest::Client::new();
    let res = client.post(API_URL)
        .json(&CreateTodo { title })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    res.json::<TodoModel>().await.map_err(|e| e.to_string())
}

pub async fn update_todo(id: i32, title: String, completed: bool) -> Result<TodoModel, String> {
    let client = reqwest::Client::new();
    let res = client.put(&format!("{}/{}", API_URL, id))
        .json(&UpdateTodo { title, completed })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    res.json::<TodoModel>().await.map_err(|e| e.to_string())
}

pub async fn delete_todo(id: i32) -> Result<(), String> {
    let client = reqwest::Client::new();
    client.delete(&format!("{}/{}", API_URL, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
