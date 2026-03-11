use leptos::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sea_orm::FromQueryResult))]
pub struct TodoModel {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[server(ListTodos)]
pub async fn list_todos() -> Result<Vec<TodoModel>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        ssr::list_todos().await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetTodo)]
pub async fn get_todo(id: i32) -> Result<Option<TodoModel>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        ssr::get_todo(id).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        unreachable!()
    }
}

#[server(CreateTodo)]
pub async fn create_todo(title: String) -> Result<TodoModel, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        ssr::create_todo(title).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = title;
        unreachable!()
    }
}

#[server(UpdateTodo)]
pub async fn update_todo(
    id: i32,
    title: String,
    completed: bool,
) -> Result<TodoModel, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        ssr::update_todo(id, title, completed).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (id, title, completed);
        unreachable!()
    }
}

#[server(DeleteTodo)]
pub async fn delete_todo(id: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        ssr::delete_todo(id).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        unreachable!()
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use super::*;
    use axum::http::StatusCode;
    use leptos::{expect_context, use_context};
    use leptos_axum::ResponseOptions;
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, ExecResult, FromQueryResult, Statement};

    fn db() -> DatabaseConnection {
        expect_context::<DatabaseConnection>()
    }

    fn statement(sql: &str, values: impl IntoIterator<Item = sea_orm::Value>) -> Statement {
        Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
    }

    fn server_error(error: sea_orm::DbErr) -> ServerFnError {
        ServerFnError::ServerError(error.to_string())
    }

    fn set_status(status: StatusCode) {
        if let Some(response) = use_context::<ResponseOptions>() {
            response.set_status(status);
        }
    }

    fn validated_title(title: String) -> Result<String, ServerFnError> {
        let title = title.trim().to_string();

        if title.is_empty() {
            return Err(ServerFnError::ServerError("Title cannot be empty".into()));
        }

        Ok(title)
    }

    pub async fn list_todos() -> Result<Vec<TodoModel>, ServerFnError> {
        TodoModel::find_by_statement(statement(
            "SELECT id, title, completed FROM todos ORDER BY id",
            [],
        ))
        .all(&db())
        .await
        .map_err(server_error)
    }

    pub async fn get_todo(id: i32) -> Result<Option<TodoModel>, ServerFnError> {
        let todo = TodoModel::find_by_statement(statement(
            "SELECT id, title, completed FROM todos WHERE id = $1",
            vec![id.into()],
        ))
        .one(&db())
        .await
        .map_err(server_error)?;

        if todo.is_none() {
            set_status(StatusCode::NOT_FOUND);
        }

        Ok(todo)
    }

    pub async fn create_todo(title: String) -> Result<TodoModel, ServerFnError> {
        let title = validated_title(title)?;

        TodoModel::find_by_statement(statement(
            "INSERT INTO todos (title, completed) VALUES ($1, FALSE) RETURNING id, title, completed",
            vec![title.into()],
        ))
        .one(&db())
        .await
        .map_err(server_error)?
        .ok_or_else(|| ServerFnError::ServerError("Failed to create todo".into()))
    }

    pub async fn update_todo(
        id: i32,
        title: String,
        completed: bool,
    ) -> Result<TodoModel, ServerFnError> {
        let title = validated_title(title)?;

        let todo = TodoModel::find_by_statement(statement(
            "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed",
            vec![title.into(), completed.into(), id.into()],
        ))
        .one(&db())
        .await
        .map_err(server_error)?;

        match todo {
            Some(todo) => Ok(todo),
            None => {
                set_status(StatusCode::NOT_FOUND);
                Err(ServerFnError::ServerError("Todo not found".into()))
            }
        }
    }

    pub async fn delete_todo(id: i32) -> Result<(), ServerFnError> {
        let result: ExecResult = db()
            .execute(statement("DELETE FROM todos WHERE id = $1", vec![id.into()]))
            .await
            .map_err(server_error)?;

        if result.rows_affected() == 0 {
            set_status(StatusCode::NOT_FOUND);
            return Err(ServerFnError::ServerError("Todo not found".into()));
        }

        Ok(())
    }
}
