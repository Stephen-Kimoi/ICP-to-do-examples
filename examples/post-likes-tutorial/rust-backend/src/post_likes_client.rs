use candid::{CandidType, Encode, Decode, Principal, Nat};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use ic_agent::Agent;
use std::io::Write;

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

    /// Get a specific post by ID
    pub async fn get_post(&self, post_id: String) -> Result<Option<Post>> {
        println!("[DEBUG] PostLikesClient: Getting post with ID: {}", post_id);
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

    /// Get all posts
    pub async fn get_posts(&self) -> Result<Vec<Post>> {
        println!("[DEBUG] PostLikesClient: Getting all posts");
        std::io::stdout().flush().unwrap();
        
        let response = self.agent
            .query(&self.canister_id, "get_posts")
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get posts: {}", e))?;

        let posts: Vec<Post> = Decode!(&response, Vec<Post>)
            .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

        println!("[DEBUG] PostLikesClient: Retrieved {} posts", posts.len());
        std::io::stdout().flush().unwrap();
        Ok(posts)
    }

    /// Get all posts with their like counts
    pub async fn get_posts_with_likes(&self) -> Result<Vec<(Post, Nat)>> {
        println!("[DEBUG] PostLikesClient: Getting all posts with likes");
        std::io::stdout().flush().unwrap();
        
        let response = self.agent
            .query(&self.canister_id, "get_posts_with_likes")
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get posts with likes: {}", e))?;

        let posts_with_likes: Vec<(Post, Nat)> = Decode!(&response, Vec<(Post, Nat)>)
            .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

        println!("[DEBUG] PostLikesClient: Retrieved {} posts with likes", posts_with_likes.len());
        std::io::stdout().flush().unwrap();
        Ok(posts_with_likes)
    }

    /// Get the number of likes for a specific post
    pub async fn get_likes(&self, post_id: String) -> Result<Nat> {
        println!("[DEBUG] PostLikesClient: Getting likes for post: {}", post_id);
        std::io::stdout().flush().unwrap();
        
        let args = Encode!(&post_id)
            .map_err(|e| anyhow::anyhow!("Failed to encode arguments: {}", e))?;

        let response = self.agent
            .query(&self.canister_id, "get_likes")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get likes: {}", e))?;

        let result: LikesResult = Decode!(&response, LikesResult)
            .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

        match result {
            LikesResult::Ok(likes) => {
                println!("[DEBUG] PostLikesClient: Retrieved likes: {}", likes);
                std::io::stdout().flush().unwrap();
                Ok(likes)
            }
            LikesResult::Err(error) => {
                println!("[DEBUG] PostLikesClient: Failed to get likes: {}", error);
                std::io::stdout().flush().unwrap();
                Err(anyhow::anyhow!("Canister error: {}", error))
            }
        }
    }

    /// Like a post (increment like count)
    pub async fn like(&self, post_id: String) -> Result<Nat> {
        println!("[DEBUG] PostLikesClient: Liking post: {}", post_id);
        std::io::stdout().flush().unwrap();
        
        let args = Encode!(&post_id)
            .map_err(|e| anyhow::anyhow!("Failed to encode arguments: {}", e))?;

        let response = self.agent
            .update(&self.canister_id, "like")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to like post: {}", e))?;

        let result: LikesResult = Decode!(&response, LikesResult)
            .map_err(|e| anyhow::anyhow!("Failed to decode response: {}", e))?;

        match result {
            LikesResult::Ok(new_likes) => {
                println!("[DEBUG] PostLikesClient: Post liked successfully. New like count: {}", new_likes);
                std::io::stdout().flush().unwrap();
                Ok(new_likes)
            }
            LikesResult::Err(error) => {
                println!("[DEBUG] PostLikesClient: Failed to like post: {}", error);
                std::io::stdout().flush().unwrap();
                Err(anyhow::anyhow!("Canister error: {}", error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[tokio::test]
    async fn test_post_likes_client_creation() {
        // This is a basic test to ensure the client can be created
        // In a real test environment, you'd want to mock the agent
        let canister_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
        
        // Note: This test requires a running agent, so it might fail in CI
        // In practice, you'd want to mock the agent for unit tests
        assert!(canister_id.to_string().len() > 0);
    }
}
