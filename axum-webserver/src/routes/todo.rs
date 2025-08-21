use crate::db::todo as db_todo;
use crate::models::todo::{NewTodo, Todo};
use axum::{extract::State, http::StatusCode, response::Json};
use deadpool_diesel::postgres::Pool;

pub async fn create_todo(
    State(pool): State<Pool>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(crate::error::internal_error)?;
    let res = conn
        .interact(move |conn| db_todo::create_todo(conn, new_todo))
        .await
        .map_err(crate::error::internal_error)?
        .map_err(crate::error::internal_error)?;
    Ok(Json(res))
}

pub async fn list_todos(State(pool): State<Pool>) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(crate::error::internal_error)?;
    let res = conn
        .interact(|conn| db_todo::list_todos(conn))
        .await
        .map_err(crate::error::internal_error)?
        .map_err(crate::error::internal_error)?;
    Ok(Json(res))
}
