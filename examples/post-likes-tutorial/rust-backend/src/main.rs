use anyhow::Result;
use url::Url;
use candid::Principal;
use serde::{Serialize, Deserialize};
use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State, Json},
    http::StatusCode,
    response::Json as JsonResponse,
};
use std::sync::Arc;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use dotenv::dotenv;
use ic_agent::Agent;

mod post_likes_client;

use post_likes_client::{PostLikesClient, Post, CreatePostRequest};



#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message,
            error: None,
        }
    }

    pub fn error(message: String, error: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
            error: Some(error),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LikesResponse {
    pub post_id: String,
    pub likes: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LikeResponse {
    pub post_id: String,
    pub new_likes: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsWithLikesResponse {
    pub posts_with_likes: Vec<(Post, String)>,
    pub message: String,
}

// Application state
#[derive(Clone)]
pub struct AppState {
    pub post_likes_client: Arc<PostLikesClient>,
}

// Health check endpoint
async fn health_check() -> JsonResponse<ApiResponse<String>> {
    JsonResponse(ApiResponse::success(
        "OK".to_string(),
        "Web2 API is running and connected to ICP".to_string(),
    ))
}

// Get likes for a specific post
async fn get_likes(
    Path(post_id): Path<String>,
    State(state): State<AppState>,
) -> Result<JsonResponse<ApiResponse<LikesResponse>>, StatusCode> {
    if post_id.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("Fetching likes for post: {}", post_id);
    
    match state.post_likes_client.get_likes(post_id.clone()).await {
        Ok(likes) => {
            let response = LikesResponse {
                post_id: post_id.clone(),
                likes: likes.to_string(),
                message: "Likes retrieved from ICP canister".to_string(),
            };
            Ok(JsonResponse(ApiResponse::success(
                response,
                "Successfully retrieved likes".to_string(),
            )))
        }
        Err(e) => {
            let response = ApiResponse::<LikesResponse>::error(
                "Failed to get likes".to_string(),
                e.to_string(),
            );
            Ok(JsonResponse(response))
        }
    }
}

// Create a new post
async fn create_post(
    State(state): State<AppState>,
    Json(request): Json<CreatePostRequest>,
) -> Result<JsonResponse<ApiResponse<Post>>, StatusCode> {
    if request.id.trim().is_empty() || request.title.trim().is_empty() || request.content.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("Creating post: {}", request.id);
    
    match state.post_likes_client.create_post(
        request.id.clone(),
        request.title.clone(),
        request.content.clone(),
    ).await {
        Ok(post) => {
            Ok(JsonResponse(ApiResponse::success(
                post,
                "Post created successfully on ICP canister".to_string(),
            )))
        }
        Err(e) => {
            let response = ApiResponse::<Post>::error(
                "Failed to create post".to_string(),
                e.to_string(),
            );
            Ok(JsonResponse(response))
        }
    }
}

// Get a specific post
async fn get_post(
    Path(post_id): Path<String>,
    State(state): State<AppState>,
) -> Result<JsonResponse<ApiResponse<Option<Post>>>, StatusCode> {
    if post_id.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("Fetching post: {}", post_id);
    
    match state.post_likes_client.get_post(post_id.clone()).await {
        Ok(post) => {
            let message = if post.is_some() {
                "Post retrieved successfully from ICP canister".to_string()
            } else {
                "Post not found".to_string()
            };
            Ok(JsonResponse(ApiResponse::success(
                post,
                message,
            )))
        }
        Err(e) => {
            let response = ApiResponse::<Option<Post>>::error(
                "Failed to get post".to_string(),
                e.to_string(),
            );
            Ok(JsonResponse(response))
        }
    }
}

// Get all posts
async fn get_posts(
    State(state): State<AppState>,
) -> Result<JsonResponse<ApiResponse<PostsResponse>>, StatusCode> {
    println!("Fetching all posts");
    
    match state.post_likes_client.get_posts().await {
        Ok(posts) => {
            let response = PostsResponse {
                posts: posts.clone(),
                message: "Posts retrieved successfully from ICP canister".to_string(),
            };
            Ok(JsonResponse(ApiResponse::success(
                response,
                "Successfully retrieved all posts".to_string(),
            )))
        }
        Err(e) => {
            let response = ApiResponse::<PostsResponse>::error(
                "Failed to get posts".to_string(),
                e.to_string(),
            );
            Ok(JsonResponse(response))
        }
    }
}

// Get all posts with likes
async fn get_posts_with_likes(
    State(state): State<AppState>,
) -> Result<JsonResponse<ApiResponse<PostsWithLikesResponse>>, StatusCode> {
    println!("Fetching all posts with likes");
    
    match state.post_likes_client.get_posts_with_likes().await {
        Ok(posts_with_likes) => {
            let response = PostsWithLikesResponse {
                posts_with_likes: posts_with_likes.iter()
                    .map(|(post, likes)| (post.clone(), likes.to_string()))
                    .collect(),
                message: "Posts with likes retrieved successfully from ICP canister".to_string(),
            };
            Ok(JsonResponse(ApiResponse::success(
                response,
                "Successfully retrieved all posts with likes".to_string(),
            )))
        }
        Err(e) => {
            let response = ApiResponse::<PostsWithLikesResponse>::error(
                "Failed to get posts with likes".to_string(),
                e.to_string(),
            );
            Ok(JsonResponse(response))
        }
    }
}

// Like a post
async fn like_post(
    Path(post_id): Path<String>,
    State(state): State<AppState>,
) -> Result<JsonResponse<ApiResponse<LikeResponse>>, StatusCode> {
    if post_id.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("Liking post: {}", post_id);
    
    match state.post_likes_client.like(post_id.clone()).await {
        Ok(new_likes) => {
            let response = LikeResponse {
                post_id: post_id.clone(),
                new_likes: new_likes.to_string(),
                message: "Post liked successfully on ICP canister".to_string(),
            };
            Ok(JsonResponse(ApiResponse::success(
                response,
                "Successfully liked post".to_string(),
            )))
        }
        Err(e) => {
            let response = ApiResponse::<LikeResponse>::error(
                "Failed to like post".to_string(),
                e.to_string(),
            );
            Ok(JsonResponse(response))
        }
    }
}

// Initialize ICP agent
async fn create_agent(url: Url, use_mainnet: bool) -> Result<Agent> {
    let agent = Agent::builder().with_url(url).build()?;
    if !use_mainnet {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Get configuration from environment variables
    let canister_id_str = std::env::var("POST_LIKES_BACKEND_CANISTER_ID")
        .expect("POST_LIKES_BACKEND_CANISTER_ID must be set in .env file");
    let canister_id = Principal::from_text(&canister_id_str)?;
    
    // Initialize ICP agent
    let dfx_network = std::env::var("DFX_NETWORK").unwrap_or_else(|_| "local".to_string());
    let icp_url = if dfx_network == "local" {
        "http://127.0.0.1:4943"
    } else {
        "https://ic0.app"
    };
    let url = Url::parse(icp_url)?;
    let agent = create_agent(url, dfx_network != "local").await?;
    
    // Create PostLikes client
    let post_likes_client = Arc::new(PostLikesClient::new(agent, canister_id));
    
    // Create app state
    let app_state = AppState {
        post_likes_client,
    };
    
    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/likes/:post_id", get(get_likes))
        .route("/posts", post(create_post))
        .route("/posts/:post_id", get(get_post))
        .route("/posts", get(get_posts))
        .route("/posts-with-likes", get(get_posts_with_likes))
        .route("/like/:post_id", post(like_post))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Starting server on http://{}", addr);
    println!("Connected to ICP canister: {}", canister_id);
    println!("Network: {}", dfx_network);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service()
    )
    .await
    .unwrap();
    
    Ok(())
}
