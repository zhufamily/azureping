use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::predictor::ModelManager;

#[derive(Clone)]
pub struct AppState {
    pub model: Arc<ModelManager>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize)]
pub struct InferenceRequest {
    // The 4 Iris features: [sepal_length, sepal_width, petal_length, petal_width]
    pub features: Vec<f32>,
}

#[derive(Serialize)]
pub struct InferenceResponse {
    pub species: String,
    pub probability: f32,
}

#[derive(serde::Deserialize)]
pub struct BatchInferenceRequest {
    pub batch: Vec<Vec<f32>>, // Expecting a list of lists
}

#[derive(serde::Serialize)]
pub struct BatchInferenceResponse {
    pub results: Vec<InferenceResponse>,
    pub count: usize, // Explicitly stating how many items were processed
    pub model_name: String,
}