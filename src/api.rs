use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::{IntoResponse, Response, Result},
    routing::post,
    Json, Router,
};
use serde::Deserialize;

use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    gpt_streaming::{stream_gpt, EvaluationResult, PartialEvaluationPayload},
    gpt_wrapper::{query_gpt, EvaluationResult as WrapperEvaluationReslut},
};

#[derive(Clone)]
struct AppState {
    full_evaluation_cache: Arc<Mutex<HashMap<String, EvaluationResult>>>,
}

pub async fn start_api() {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    let state = AppState {
        full_evaluation_cache: Arc::new(Mutex::new(HashMap::new())),
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_headers(Any)
        .allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", post(page_evaluation))
        .route("/streaming/evaluation", post(streaming_evaluation))
        .route("/streaming/complete", post(complete_evaluation))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct PageEvaluationPayload {
    page_body: String,
    request_id: String,
}

#[derive(Deserialize)]
struct CompletePageEvaluationPayload {
    request_id: String,
}

struct GptPromptError;

impl IntoResponse for GptPromptError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.").into_response()
    }
}

async fn page_evaluation(
    Json(payload): Json<PageEvaluationPayload>,
) -> Result<Json<WrapperEvaluationReslut>, GptPromptError> {
    match query_gpt(payload.page_body).await {
        Ok(solution) => Ok(Json(solution)),
        Err(e) => {
            println!("Couldn't fetch because of error: {}", e);
            Err(GptPromptError {})
        }
    }
}

async fn complete_evaluation(
    State(state): State<AppState>,
    Json(payload): Json<CompletePageEvaluationPayload>,
) -> Result<Json<EvaluationResult>, GptPromptError> {
    match state
        .full_evaluation_cache
        .lock()
        .unwrap()
        .get(&payload.request_id)
    {
        Some(e) => Ok(Json(e.clone())),
        None => Err(GptPromptError),
    }
}

async fn streaming_evaluation(
    State(state): State<AppState>,
    Json(payload): Json<PageEvaluationPayload>,
) -> Result<Json<PartialEvaluationPayload>, GptPromptError> {
    let (evaluation_sender, mut evaluation_receiver) =
        mpsc::channel::<(String, EvaluationResult)>(1);

    tokio::spawn(async move {
        while let Some(evaluation) = evaluation_receiver.recv().await {
            println!("Received full evaluation {:?}", evaluation);
            state
                .full_evaluation_cache
                .lock()
                .unwrap()
                .insert(evaluation.0, evaluation.1);
        }
    });

    match stream_gpt(payload.page_body, payload.request_id, evaluation_sender).await {
        Ok(eval) => Ok(Json(eval)),
        Err(_) => todo!(),
    }
}
