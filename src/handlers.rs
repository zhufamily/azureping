use axum::extract::{Path, State};
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;
use crate::models::{BatchInferenceRequest, BatchInferenceResponse, CreateUser, InferenceRequest, InferenceResponse, User};

pub async fn root() -> &'static str {
    "Welcome! This is the public root with some AI predictions."
}

pub async fn create_user(
    payload: Result<Json<CreateUser>, JsonRejection>,
) -> impl IntoResponse {
    match payload {
        Ok(Json(data)) => {
            let user = User {
                id: 1, // In reality, this would come from a DB
                username: data.username,
            };
            (StatusCode::CREATED, Json(user)).into_response()
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            format!("Invalid JSON payload: {}", err.body_text()),
        ).into_response(),
    }
}

pub async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {
    Json(User {
        id,
        username: format!("user_{}", id),
    })
}

pub async fn inference_handler(
    State(state): State<AppState>,
    Json(payload): Json<InferenceRequest>,
) -> Json<InferenceResponse> {
    let (species, probability) = state.model.predict(payload.features);

    Json(InferenceResponse {
        species,
        probability
    })
}

pub async fn batch_inference_handler(
    State(state): State<AppState>,
    Json(payload): Json<BatchInferenceRequest>,
) -> Json<BatchInferenceResponse> {
    let response = state.model.predict_batch(payload.batch);
    Json(response)
}