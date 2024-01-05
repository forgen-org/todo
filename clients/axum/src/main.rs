mod runtime;

use application::{command::Command, port::*, query::GetTodoListQuery};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
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
        .route("/todolist", post(handle_command))
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

async fn handle_command(
    State(state): State<Arc<Runtime>>,
    Json(command): Json<Command>,
) -> Result<(), AppError> {
    command.execute(state.as_ref()).await?;
    Ok(())
}

struct AppError(AnyError);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<AnyError>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
