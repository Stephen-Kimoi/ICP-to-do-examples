# Post Likes Rust Backend - Code Implementation Guide

This guide explains how to build a Rust Web2 API that integrates with an ICP canister using the official Rust agent. We'll walk through the code structure and show you exactly how to call your backend canister from a Rust service.

## 🏗️ Architecture Overview

The Rust backend consists of two main files:
- **`main.rs`** - Web server setup, routing, and HTTP handlers
- **`post_likes_client.rs`** - ICP agent client for canister communication

## 📁 Code Structure

### 1. Main Application (`main.rs`)

#### **Data Structures and API Responses**
```rust
// Data structures for API responses
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreatePostRequest {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub error: Option<String>,
}
```
*Source: [main.rs lines 25-75](src/main.rs#L25-L75)*

The `ApiResponse<T>` wrapper provides consistent error handling across all endpoints. It includes:
- `success`: Boolean indicating operation success
- `data`: Optional response data of type T
- `message`: Human-readable status message
- `error`: Optional error details

#### **Application State**
```rust
// Application state
#[derive(Clone)]
pub struct AppState {
    pub post_likes_client: Arc<PostLikesClient>,
}
```
*Source: [main.rs lines 77-81](src/main.rs#L77-L81)*

The `AppState` holds a shared reference to the `PostLikesClient`, which is wrapped in an `Arc` for thread-safe sharing across async handlers.

#### **ICP Agent Initialization**
```rust
// Initialize ICP agent
async fn create_agent(url: Url, use_mainnet: bool) -> Result<Agent> {
    let agent = Agent::builder().with_url(url).build()?;
    if !use_mainnet {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}
```
*Source: [main.rs lines 280-290](src/main.rs#L280-L290)*

This function creates the ICP agent and fetches the root key for local development. The root key is required for local canister calls.

#### **Environment Configuration**
```rust
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
```
*Source: [main.rs lines 295-305](src/main.rs#L295-L305)*

The application reads the canister ID and network configuration from environment variables, automatically switching between local development and mainnet URLs.

#### **Route Handlers**

Each handler follows the same pattern: validate input, call the canister via the client, and return a formatted response.

**Health Check Handler:**
```rust
// Health check endpoint
async fn health_check() -> JsonResponse<ApiResponse<String>> {
    JsonResponse(ApiResponse::success(
        "OK".to_string(),
        "Web2 API is running and connected to ICP".to_string(),
    ))
}
```
*Source: [main.rs lines 95-100](src/main.rs#L95-L100)*

**Get Likes Handler:**
```rust
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
```
*Source: [main.rs lines 102-125](src/main.rs#L102-L125)*

**Create Post Handler:**
```rust
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
```
*Source: [main.rs lines 127-155](src/main.rs#L127-L155)*

#### **Router Setup**
```rust
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
```
*Source: [main.rs lines 320-330](src/main.rs#L320-L330)*

The router maps HTTP endpoints to handler functions and applies CORS and tracing middleware.

### 2. ICP Client (`post_likes_client.rs`)

#### **Data Structures**
```rust
// Data structures matching the Candid interface
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: u64,
}

// Result types from the canister
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum PostResult {
    Ok(Post),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum LikesResult {
    Ok(Nat),
    Err(String),
}
```
*Source: [post_likes_client.rs lines 8-25](src/post_likes_client.rs#L8-L25)*

These structures match the Candid interface of your canister. The `CandidType` derive macro enables automatic serialization/deserialization for ICP communication.

#### **Client Implementation**
```rust
/// Client for interacting with the post-likes backend canister
pub struct PostLikesClient {
    agent: Agent,
    canister_id: Principal,
}

impl PostLikesClient {
    /// Create a new PostLikesClient
    pub fn new(agent: Agent, canister_id: Principal) -> Self {
        Self {
            agent,
            canister_id,
        }
    }
```
*Source: [post_likes_client.rs lines 27-35](src/post_likes_client.rs#L27-L35)*

The `PostLikesClient` wraps the ICP agent and canister ID, providing a clean interface for canister operations.

#### **Canister Method Calls**

**Create Post (Update Call):**
```rust
/// Create a new post
pub async fn create_post(&self, id: String, title: String, content: String) -> Result<Post> {
    println!("[DEBUG] PostLikesClient: Creating post with ID: {}", id);
    std::io::stdout().flush().unwrap();
    
    let args = Encode!(&id, &title, &content)
        .map_err(|e| anyhow::anyhow!("Failed to encode arguments: {}", e))?;

    let response = self.agent
        .update(&self.canister_id, "create_post")
        .with_arg(args)
        .call_and_wait()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create post: {}", e))?;

    let result: PostResult = Decode!(&response, PostResult)
        .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

    match result {
        PostResult::Ok(post) => {
            println!("[DEBUG] PostLikesClient: Post created successfully: {:?}", post);
            std::io::stdout().flush().unwrap();
            Ok(post)
        }
        PostResult::Err(error) => {
            println!("[DEBUG] PostLikesClient: Failed to create post: {}", error);
            std::io::stdout().flush().unwrap();
            Err(anyhow::anyhow!("Canister error: {}", error))
        }
    }
}
```
*Source: [post_likes_client.rs lines 37-65](src/post_likes_client.rs#L37-L65)*

Key points:
- **`Encode!`**: Serializes arguments into Candid format
- **`update()`**: Calls a canister update method (modifies state)
- **`call_and_wait()`**: Waits for the update to complete
- **`Decode!`**: Deserializes the response from Candid format

**Get Post (Query Call):**
```rust
/// Get a specific post by ID
pub async fn get_post(&self, post_id: String) -> Result<Option<Post>> {
    println!("[DEBUG] PostLikesClient: Creating post with ID: {}", post_id);
    std::io::stdout().flush().unwrap();
    
    let args = Encode!(&post_id)
        .map_err(|e| anyhow::anyhow!("Failed to encode arguments: {}", e))?;

    let response = self.agent
        .query(&self.canister_id, "get_post")
        .with_arg(args)
        .call()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get post: {}", e))?;

    let post: Option<Post> = Decode!(&response, Option<Post>)
        .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

    println!("[DEBUG] PostLikesClient: Retrieved post: {:?}", post);
    std::io::stdout().flush().unwrap();
    Ok(post)
}
```
*Source: [post_likes_client.rs lines 67-90](src/post_likes_client.rs#L67-L90)*

Key differences from update calls:
- **`query()`**: Calls a canister query method (read-only)
- **`call()`**: Immediate response, no waiting
- **No state modification**: Queries don't change canister state

**Get Posts with Likes:**
```rust
/// Get all posts with their like counts
pub async fn get_posts_with_likes(&self) -> Result<Vec<(Post, Nat)>> {
    println!("[DEBUG] PostLikesClient: Getting all posts with likes");
    std::io::stdout().flush().unwrap();
    
    // Encode empty tuple as argument for methods that take ()
    let args = Encode!(&())
        .map_err(|e| anyhow::anyhow!("Failed to encode empty arguments: {}", e))?;
    
    let response = self.agent
        .query(&self.canister_id, "get_posts_with_likes")
        .with_arg(args)
        .call()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get posts with likes: {}", e))?;

    let posts_with_likes: Vec<(Post, Nat)> = Decode!(&response, Vec<(Post, Nat)>)
        .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

    println!("[DEBUG] PostLikesClient: Retrieved {} posts with likes", posts_with_likes.len());
    std::io::stdout().flush().unwrap();
    Ok(posts_with_likes)
}
```
*Source: [post_likes_client.rs lines 130-150](src/post_likes_client.rs#L130-L150)*

Note the `Encode!(&())` for methods that take no arguments - this encodes an empty tuple.

## 🔑 Key Concepts for ICP Integration

### **Update vs Query Calls**
- **Update calls** (`update()` + `call_and_wait()`): Modify canister state, require consensus
- **Query calls** (`query()` + `call()`): Read-only, immediate response

### **Argument Encoding**
- Use `Encode!()` to serialize arguments into Candid format
- Use `Decode!()` to deserialize responses
- Handle empty arguments with `Encode!(&())`

### **Error Handling**
- Canister methods return `Result` types (e.g., `PostResult`, `LikesResult`)
- Always match on the result to handle success/error cases
- Use `anyhow::anyhow!` for consistent error wrapping

### **Async Operations**
- All canister calls are async
- Use `.await` to wait for responses
- Update calls require `call_and_wait()` for consensus

## 🚀 Running the Service

### Development Mode
```bash
cargo run
```

### Production Build
```bash
cargo build --release
./target/release/post-likes-rust-backend
```

## 🔧 Configuration

Create a `.env` file with:
```env
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=your-canister-id
```

## 📚 Next Steps

After understanding this implementation:
1. **Customize the data structures** to match your canister interface
2. **Add authentication** to your handlers
3. **Implement caching** for frequently accessed data
4. **Add metrics and monitoring** for canister calls
5. **Deploy to production** with proper error handling

This implementation demonstrates the power of Rust's type safety combined with ICP's decentralized capabilities, providing a robust foundation for building Web2 APIs that integrate seamlessly with the Internet Computer.
