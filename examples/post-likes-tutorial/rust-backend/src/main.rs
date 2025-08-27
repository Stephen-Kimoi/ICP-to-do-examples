use anyhow::Result;
use url::Url;
use candid::Principal;
use std::sync::Arc;
use ic_agent::Agent;

mod post_likes_client;

use post_likes_client::PostLikesClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::dotenv().ok();
    
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
    
    println!("🚀 Post Likes Rust Client Started");
    println!("Connected to ICP canister: {}", canister_id);
    println!("Network: {}", dfx_network);
    println!("");
    
    // Test the client
    test_post_likes_client(&post_likes_client).await?;
    
    Ok(())
}

// Initialize ICP agent
async fn create_agent(url: Url, use_mainnet: bool) -> Result<Agent> {
    let agent = Agent::builder().with_url(url).build()?;
    if !use_mainnet {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}

// Simple test function to demonstrate the client
async fn test_post_likes_client(client: &Arc<PostLikesClient>) -> Result<()> {
    println!("🧪 Testing Post Likes Client");
    println!("============================");
    
    // Test 1: Get all posts
    println!("\n[DEBUG] ====== Getting All Posts ======");
    match client.get_posts().await {
        Ok(posts) => {
            println!("[DEBUG] Retrieved {} posts successfully", posts.len());
            for post in &posts {
                println!("  - {}: {} ({})", post.id, post.title, post.content);
            }
        }
        Err(e) => {
            println!("[DEBUG] Failed to get posts: {}", e);
        }
    }
    
    // Test 2: Get posts with likes
    println!("\n[DEBUG] ====== Getting Posts with Likes ======");
    match client.get_posts_with_likes().await {
        Ok(posts_with_likes) => {
            println!("[DEBUG] Retrieved {} posts with likes successfully", posts_with_likes.len());
            for (post, likes) in &posts_with_likes {
                println!("  - {}: {} ({} likes)", post.id, post.title, likes);
            }
        }
        Err(e) => {
            println!("[DEBUG] Failed to get posts with likes: {}", e);
        }
    }
    
    // Test 3: Create a new post
    println!("\n[DEBUG] ====== Creating New Post ======");
    match client.create_post(
        "test-post-rust".to_string(),
        "Test Post from Rust Client".to_string(),
        "This is a test post created by the Rust client.".to_string(),
    ).await {
        Ok(post) => {
            println!("[DEBUG] Post created successfully: {} - {}", post.id, post.title);
        }
        Err(e) => {
            println!("[DEBUG] Failed to create post: {}", e);
        }
    }
    
    // Test 4: Like a post
    println!("\n[DEBUG] ====== Liking Post ======");
    match client.like("post-1".to_string()).await {
        Ok(new_likes) => {
            println!("[DEBUG] Post liked successfully. New like count: {}", new_likes);
        }
        Err(e) => {
            println!("[DEBUG] Failed to like post: {}", e);
        }
    }
    
    // Test 5: Get likes for a specific post
    println!("\n[DEBUG] ====== Getting Likes for Post ======");
    match client.get_likes("post-1".to_string()).await {
        Ok(likes) => {
            println!("[DEBUG] Post has {} likes", likes);
        }
        Err(e) => {
            println!("[DEBUG] Failed to get likes: {}", e);
        }
    }
    
    println!("\n🎉 Testing completed!");
    Ok(())
}
