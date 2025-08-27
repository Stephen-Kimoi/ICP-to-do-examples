use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use candid::{Encode, Principal, CandidType};
use ic_agent::{Agent, identity::AnonymousIdentity};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn, error};
use url::Url;

// Types for our API
#[derive(Serialize, Deserialize, CandidType)]
struct Post {
    id: String,
    title: String,
    content: String,
    created_at: u64,
}

#[derive(Deserialize, CandidType)]
struct PostWithLikes(Post, u64);

#[derive(Serialize, Deserialize)]
struct CreatePostRequest {
    id: String,
    title: String,
    content: String,
}

// App state
struct AppState {
    agent: Agent,
    canister_id: Principal,
}

// Initialize the ICP agent
async fn create_agent(url: Url, use_mainnet: bool) -> Result<Agent> {
    let agent = Agent::builder().with_url(url).build()?;
    if !use_mainnet {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}

// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Rust backend is running and connected to ICP"
    }))
}

// Get likes for a post
async fn get_likes(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if post_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let encoded_arg = Encode!(&post_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match state.agent.query(&state.canister_id, "get_likes")
        .with_arg(encoded_arg)
        .call()
        .await {
        Ok(response) => {
            // Just return the raw response like the Python backend does
            Ok(Json(serde_json::json!({
                "post_id": post_id,
                "result": response,
                "message": "Likes retrieved from ICP canister"
            })))
        }
        Err(err) => {
            warn!("Canister call failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Like a post
async fn like_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if post_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let encoded_arg = Encode!(&post_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match state.agent.update(&state.canister_id, "like")
        .with_arg(encoded_arg)
        .call_and_wait()
        .await {
        Ok(response) => {
            // Just return the raw response like the Python backend does
            Ok(Json(serde_json::json!({
                "post_id": post_id,
                "result": response,
                "message": "Post liked successfully on ICP canister"
            })))
        }
        Err(err) => {
            warn!("Canister call failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get all posts with likes
async fn get_posts(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Received request for posts");
    let empty_args = Encode!(&()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    info!("Encoded empty args successfully");
    match state.agent.query(&state.canister_id, "get_posts_with_likes")
        .with_arg(empty_args)
        .call()
        .await {
        Ok(response) => {
            info!("Received response from canister: {:?}", response);
            
            // Just return the raw response like the Python backend does
            Ok(Json(serde_json::json!({
                "posts": response,
                "message": "Posts retrieved directly from ICP canister"
            })))
        }
        Err(err) => {
            error!("Canister call failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Create a new post
async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePostRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if request.id.is_empty() || request.title.is_empty() || request.content.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let args = (request.id.clone(), request.title.clone(), request.content.clone());
    
    let encoded_args = Encode!(&args).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match state.agent.update(&state.canister_id, "create_post")
        .with_arg(encoded_args)
        .call_and_wait()
        .await {
        Ok(response) => {
            // Just return the raw response like the Python backend does
            Ok(Json(serde_json::json!({
                "result": response,
                "message": "Post created successfully on ICP canister"
            })))
        }
        Err(err) => {
            warn!("Canister call failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get configuration from environment
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let dfx_network = std::env::var("DFX_NETWORK").unwrap_or_else(|_| "local".to_string());
    let canister_id = std::env::var("POST_LIKES_BACKEND_CANISTER_ID")
        .expect("POST_LIKES_BACKEND_CANISTER_ID must be set");

    // Parse canister ID
    let canister_principal = Principal::from_text(&canister_id)?;

    // Initialize agent
    let url = if dfx_network == "local" {
        Url::parse("http://127.0.0.1:4943")?
    } else {
        Url::parse("https://ic0.app")?
    };

    let mut agent = create_agent(url, dfx_network != "local").await?;
    
    // Use anonymous identity for this example
    // In production, you'd want to implement proper identity management
    let identity = AnonymousIdentity;
    agent.set_identity(identity);

    info!("Rust backend initialized with canister ID: {}", canister_id);

    // Create app state
    let state = Arc::new(AppState {
        agent,
        canister_id: canister_principal,
    });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/likes/:post_id", get(get_likes))
        .route("/like/:post_id", post(like_post))
        .route("/posts", get(get_posts))
        .route("/posts", post(create_post))
        .layer(cors)
        .with_state(state);

    info!("Starting Rust backend server on port {}", port);
    
    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}