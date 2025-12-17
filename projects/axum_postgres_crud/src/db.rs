use axum::{
	Json, Router,
	extract::{Path, State},
	http::StatusCode,
	routing::{get, patch},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::net::TcpListener;

#[derive(Serialize)]
pub struct TaskRow {
	task_id: i32,
	name: String,
	priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
	name: String,
	priority: Option<i32>,
}

#[derive(Serialize)]
pub struct CreateTaskRow {
	task_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateTaskReq {
	name: Option<String>,
	priority: Option<i32>,
}

fn to_error_response(e: sqlx::Error) -> (StatusCode, String) {
	(
		StatusCode::INTERNAL_SERVER_ERROR,
		json!({"success": false, "message": e.to_string()}).to_string(),
	)
}

fn to_get_tasks_success(rows: Vec<TaskRow>) -> (StatusCode, String) {
	(
		StatusCode::OK,
		json!({"success": true, "data": rows}).to_string(),
	)
}

fn to_create_task_success(row: CreateTaskRow) -> (StatusCode, String) {
	(
		StatusCode::OK,
		json!({"success": true, "data": row}).to_string(),
	)
}

fn to_update_task_success() -> (StatusCode, String) {
	(StatusCode::OK, json!({"success": true}).to_string())
}

fn to_delete_task_success() -> (StatusCode, String) {
	(StatusCode::OK, json!({"success": true}).to_string())
}

pub async fn get_tasks(
	State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
	let rows: Vec<TaskRow> = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
		.fetch_all(&db_pool)
		.await
		.map_err(to_error_response)?;
	Ok(to_get_tasks_success(rows))
}

pub async fn create_task(
	State(db_pool): State<PgPool>,
	Json(task): Json<CreateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
	let row: CreateTaskRow = sqlx::query_as!(
		CreateTaskRow,
		"INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
		task.name,
		task.priority
	)
	.fetch_one(&db_pool)
	.await
	.map_err(to_error_response)?;
	Ok(to_create_task_success(row))
}

pub async fn update_task(
	State(db_pool): State<PgPool>,
	Path(task_id): Path<i32>,
	Json(task): Json<UpdateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
	let mut query = "UPDATE tasks SET task_id = $1".to_owned();
	let mut i = 2;
	if task.name.is_some() {
		query.push_str(&format!(", name = ${i}"));
		i = i + 1;
	};
	if task.priority.is_some() {
		query.push_str(&format!(", priority = ${i}"));
	};
	query.push_str(&format!(" WHERE task_id = $1"));
	let mut s = sqlx::query(&query).bind(task_id);

	if task.name.is_some() {
		s = s.bind(task.name);
	}

	if task.priority.is_some() {
		s = s.bind(task.priority);
	}
	s.execute(&db_pool).await.map_err(to_error_response)?;
	Ok(to_update_task_success())
}

pub async fn delete_task(
	State(db_pool): State<PgPool>,
	Path(task_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
	sqlx::query!("DELETE from tasks WHERE task_id = $1", task_id)
		.execute(&db_pool)
		.await
		.map_err(to_error_response)?;
	Ok(to_delete_task_success())
}
