mod models;
mod middleware;
mod handlers;
mod predictor;

use axum::{
    middleware as axum_middleware,
    routing::{get, post}, Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use crate::predictor::ModelManager;
// --- Models ---
// moved to models.rs

// State Manager
#[derive(Clone)]
pub struct AppState {
    pub model: Arc<ModelManager>,
}

// --- Main Server Setup ---
#[tokio::main]
async fn main() {
    // Load .env file for local testing (Optional: add dotenvy to Cargo.toml)
    let _ = dotenvy::dotenv();
    // Init logger
    env_logger::init();

    // load model
    let model_path = resolve_model_path();
    let model_manager = ModelManager::new(&model_path);
    let state = AppState {
        model: Arc::new(model_manager),
    };

    // User Routes (require PCX_AUTH_KEY)
    let user_routes = Router::new()
        .route("/", post(handlers::create_user))
        .route("/{id}", get(handlers::get_user))
        .route_layer(axum_middleware::from_fn(middleware::auth_middleware));

    // Predict Routes
    let predict_routes = Router::new()
        .route("/", post(handlers::inference_handler))
        .route("/batch", post(handlers::batch_inference_handler));

    // 2. Main Router
    let app = Router::new()
        .route("/", get(handlers::root))
        .nest("/users", user_routes)
        .nest("/predict", predict_routes)
        .with_state(state);

    // Define address
    let addr = SocketAddr::from(([0, 0, 0, 0], 1024));
    log::info!("🚀 Server started at http://{}", addr);
    // println!("🚀 Server started at http://{}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

fn resolve_model_path() -> PathBuf {
    let model_dir = std::env::var("MODEL_DIR").unwrap_or_else(|_| "models".to_string());
    PathBuf::from(model_dir).join("irismodel.onnx")
}

// Gracefully shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { println!("Shutdown signal received: Ctrl+C"); },
        _ = terminate => { println!("Shutdown signal received: SIGTERM (Azure/Docker)"); },
    }

    println!("⏳ Gracefully shutting down... finishing active requests.");
}

// --- Middleware ---
// moved to middleware.rs

// --- Handlers ---
// moved to handlers.rs