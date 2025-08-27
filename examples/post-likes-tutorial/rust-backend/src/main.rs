use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use candid::{Decode, Encode, Principal, CandidType};
use ic_agent::{Agent, identity::AnonymousIdentity};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};
use url::Url;

// Types for our API
#[derive(Serialize, Deserialize, CandidType)]
struct Post {
    id: String,
    title: String,
    content: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize)]
struct CreatePostRequest {
    id: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    data: Option<T>,
    message: String,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LikesResponse {
    post_id: String,
    likes: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct LikeResponse {
    post_id: String,
    new_likes: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct PostsResponse {
    posts: Vec<Post>,
    message: String,
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
async fn health_check() -> Json<ApiResponse<()>> {
    Json(ApiResponse {
        data: None,
        message: "Rust backend is running and connected to ICP".to_string(),
        error: None,
    })
}

// Get likes for a post
async fn get_likes(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Json<LikesResponse>, StatusCode> {
    if post_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let encoded_arg = Encode!(&post_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match state.agent.query(&state.canister_id, "get_likes")
        .with_arg(encoded_arg)
        .call()
        .await {
        Ok(response) => {
            let result: Result<String, String> = Decode!(&response, Result<String, String>)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            match result {
                Ok(likes) => Ok(Json(LikesResponse {
                    post_id,
                    likes,
                    message: "Likes retrieved from ICP canister".to_string(),
                })),
                Err(err) => {
                    warn!("Failed to get likes: {}", err);
                    Err(StatusCode::BAD_REQUEST)
                }
            }
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
) -> Result<Json<LikeResponse>, StatusCode> {
    if post_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let encoded_arg = Encode!(&post_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match state.agent.update(&state.canister_id, "like")
        .with_arg(encoded_arg)
        .call_and_wait()
        .await {
        Ok(response) => {
            let result: Result<String, String> = Decode!(&response, Result<String, String>)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            match result {
                Ok(new_likes) => Ok(Json(LikeResponse {
                    post_id,
                    new_likes,
                    message: "Post liked successfully on ICP canister".to_string(),
                })),
                Err(err) => {
                    warn!("Failed to like post: {}", err);
                    Err(StatusCode::BAD_REQUEST)
                }
            }
        }
        Err(err) => {
            warn!("Canister call failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get all posts with likes
async fn get_posts(State(state): State<Arc<AppState>>) -> Result<Json<PostsResponse>, StatusCode> {
    match state.agent.query(&state.canister_id, "get_posts_with_likes")
        .call()
        .await {
        Ok(response) => {
            let posts: Vec<Post> = Decode!(&response, Vec<Post>)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(PostsResponse {
                posts,
                message: "Posts retrieved directly from ICP canister".to_string(),
            }))
        }
        Err(err) => {
            warn!("Canister call failed: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Create a new post
async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, StatusCode> {
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
            let result: Result<Post, String> = Decode!(&response, Result<Post, String>)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            match result {
                Ok(post) => Ok(Json(ApiResponse {
                    data: Some(post),
                    message: "Post created successfully on ICP canister".to_string(),
                    error: None,
                })),
                Err(err) => {
                    warn!("Failed to create post: {}", err);
                    Err(StatusCode::BAD_REQUEST)
                }
            }
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