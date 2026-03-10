use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::*;
use serde::Deserialize;

use crate::{entity::todo, AppState};

#[derive(Deserialize)]
pub struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    title: String,
    completed: bool,
}

pub async fn list_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<todo::Model>>, (StatusCode, String)> {
    let todos = todo::Entity::find()
        .order_by_asc(todo::Column::Id)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todos))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<todo::Model>), (StatusCode, String)> {
    let todo = todo::ActiveModel {
        title: Set(payload.title.to_owned()),
        completed: Set(false),
        ..Default::default()
    };

    let result = todo
        .insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<todo::Model>, (StatusCode, String)> {
    let todo = todo::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(todo) = todo {
        Ok(Json(todo))
    } else {
        Err((StatusCode::NOT_FOUND, "Todo not found".into()))
    }
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<todo::Model>, (StatusCode, String)> {
    let mut todo: todo::ActiveModel = todo::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Todo not found".into()))?
        .into();

    todo.title = Set(payload.title.to_owned());
    todo.completed = Set(payload.completed);

    let updated = todo
        .update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let res = todo::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if res.rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Todo not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}
