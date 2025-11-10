mod api;
mod chat;
mod config;
mod decision;
mod dialog;
mod experience;
mod memory;
mod middleware;
mod ollama;
mod pattern;
mod personality;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post},
    Router,
};
use config::Config;
use memory::{Memory, SharedMemory};
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_core=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration from .env");
    tracing::info!("🚀 Starting AI Core API");
    tracing::info!("   Bearer Token configured: {}", !config.bearer_token.is_empty());
    tracing::info!("   Ollama enabled: {}", config.ollama_enabled);
    if config.ollama_enabled {
        tracing::info!("   Ollama URL: {}", config.ollama_url);
        tracing::info!("   Ollama Model: {}", config.ollama_model);
    }

    // Initialize Ollama client
    let ollama_client = Arc::new(ollama::OllamaClient::new(
        config.ollama_url.clone(),
        config.ollama_model.clone(),
        config.ollama_enabled,
    ));

    // Health check Ollama if enabled
    if config.ollama_enabled {
        tracing::info!("   Checking Ollama connection...");
        if ollama_client.health_check().await {
            tracing::info!("   ✅ Ollama is running and accessible");
            
            // List available models
            match ollama_client.list_models().await {
                Ok(models) => {
                    tracing::info!("   Available models: {}", models.join(", "));
                }
                Err(e) => {
                    tracing::warn!("   Failed to list models: {}", e);
                }
            }
        } else {
            tracing::warn!("   ⚠️  Ollama is not accessible. Chat will use fallback responses.");
            tracing::warn!("   Make sure Ollama is running: ollama serve");
        }
    }

    // Initialize shared memory
    let memory: SharedMemory = Arc::new(RwLock::new(Memory::new()));

    // Try to load existing memory
    if let Ok(loaded_memory) = Memory::load_from_file("data/memory.json") {
        *memory.write().unwrap() = loaded_memory;
        tracing::info!("   Loaded existing memory from file");
    } else {
        tracing::info!("   Starting with fresh memory");
    }

    // Create application state
    let app_state = api::AppState::new(memory.clone(), ollama_client.clone());

    // Build protected routes (require authentication)
    let protected_routes = Router::new()
        .route("/experiences", get(api::get_experiences))
        .route("/experiences/:id", get(api::get_experience_by_id))
        .route("/experiences", post(api::create_experience))
        .route("/experiences/search", get(api::search_experiences))
        .route("/stats", get(api::get_stats))
        .route("/patterns/:keyword", get(api::get_pattern_detail))
        .route("/patterns/clear", post(api::clear_patterns))
        .route("/decision", get(api::make_decision))
        .route("/decision/query", get(api::make_decision_for_query))
        .route("/interact", get(api::interact_with_ai))
        .route("/personality", post(api::update_personality))
        .route("/reflect", get(api::reflect_memory))
        .route("/memory/clear", delete(api::clear_memory))
        // Chat endpoints
        .route("/chat/send", post(api::send_chat_message))
        .route("/chat/history/:session_id", get(api::get_chat_history))
        .route("/chat/sessions", get(api::list_chat_sessions))
        .route("/chat/sessions/:session_id", delete(api::clear_chat_session))
        .route("/chat/upload", post(api::upload_document))
        .route("/chat/export", get(api::export_chat_session))
        // API Learning CRUD endpoints
        .route("/api-learning/execute", post(api::execute_http_request))
        .route("/api-learning/records", get(api::get_learning_records))
        .route("/api-learning/records/:id", get(api::get_learning_record_by_id))
        .route("/api-learning/records/:id", post(api::update_learning_record))
        .route("/api-learning/records/:id", delete(api::delete_learning_record))
        .route("/api-learning/search", get(api::search_learning_records))
        .route("/api-learning/clear", delete(api::clear_learning_records))
        .layer(axum_middleware::from_fn(middleware::auth_middleware));

    // Build public routes (no authentication)
    let public_routes = Router::new()
        .route("/", get(|| async { "AI Core API v0.1.0" }))
        .route("/health", get(api::health_check));

    // Combine routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Clone memory for background save task
    let memory_for_save = memory.clone();
    
    // Spawn background task to periodically save memory
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Ok(mem) = memory_for_save.read() {
                if let Err(e) = mem.save_to_file("data/memory.json") {
                    tracing::error!("Failed to save memory: {}", e);
                } else {
                    tracing::debug!("Memory saved to file");
                }
            }
        }
    });

    // Start server
    let addr = config.address();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("   API listening on http://{}", addr);
    tracing::info!("   Use Bearer token in Authorization header");
    tracing::info!("\n📝 Example request:");
    tracing::info!("   curl -H 'Authorization: Bearer {}' http://{}/health", config.bearer_token, addr);
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
