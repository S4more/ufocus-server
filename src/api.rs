use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Result},
    routing::post,
    Json, Router,
};
use serde::Deserialize;

use crate::gpt_wrapper::{query_gpt, EvaluationResult};

pub async fn start_api() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", post(page_evaluation));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct PageEvaluationPayload {
    page_body: String,
}

struct GptPromptError;

impl IntoResponse for GptPromptError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.").into_response()
    }
}

async fn page_evaluation(
    Json(payload): Json<PageEvaluationPayload>,
) -> Result<Json<EvaluationResult>, GptPromptError> {
    match query_gpt(payload.page_body).await {
        Ok(solution) => Ok(Json(solution)),
        Err(e) => {
            println!("Couldn't fetch because of error: {}", e);
            Err(GptPromptError {})
        }
    }
}
