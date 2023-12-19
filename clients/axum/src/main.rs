mod runtime;

use application::{
    commands::{AddTaskCommand, CompleteTaskCommand, RemoveTaskCommand},
    ports::*,
    queries::GetTodoListQuery,
};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use framework::*;
use runtime::Runtime;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let runtime = Arc::new(Runtime::default());

    let app = Router::new()
        .route("/todolist", get(get_todolist))
        .route("/todolist/add", post(handle_command::<AddTaskCommand>))
        .route(
            "/todolist/remove",
            post(handle_command::<RemoveTaskCommand>),
        )
        .route(
            "/todolist/complete",
            post(handle_command::<CompleteTaskCommand>),
        )
        .with_state(runtime);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_todolist(
    State(state): State<Arc<Runtime>>,
) -> Result<Json<TodoListProjection>, String> {
    let query = GetTodoListQuery {};
    query
        .execute(state.as_ref())
        .await
        .map(Json)
        .map_err(|err| err.to_string())
}

async fn handle_command<T>(
    State(state): State<Arc<Runtime>>,
    Json(command): Json<T>,
) -> Result<(), String>
where
    T: Command<Runtime>,
{
    command
        .execute(state.as_ref())
        .await
        .map_err(|err| err.to_string())
}
