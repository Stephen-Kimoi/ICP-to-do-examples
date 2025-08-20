use candid::{Nat, CandidType};
use serde::Deserialize;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct Post {
    id: String,
    title: String,
    content: String,
    created_at: u64,
}

thread_local! {
    static POSTS: RefCell<HashMap<String, Post>> = RefCell::new(HashMap::new());
    static LIKES: RefCell<HashMap<String, Nat>> = RefCell::new(HashMap::new());
}

// Initialize posts when the canister is deployed
#[ic_cdk::init]
fn init() {
    let sample_posts = vec![
        Post {
            id: "post-1".to_string(),
            title: "Getting Started with ICP".to_string(),
            content: "Learn the basics of Internet Computer Protocol and how to build decentralized applications.".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Post {
            id: "post-2".to_string(),
            title: "Web2 + ICP Integration".to_string(),
            content: "Bridge the gap between traditional web applications and decentralized infrastructure.".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Post {
            id: "post-3".to_string(),
            title: "Tamper-Proof Data".to_string(),
            content: "Why decentralization matters and how ICP ensures data integrity and transparency.".to_string(),
            created_at: ic_cdk::api::time(),
        },
    ];

    POSTS.with(|posts| {
        let mut posts = posts.borrow_mut();
        for post in sample_posts {
            posts.insert(post.id.clone(), post);
        }
    });

    // Initialize likes for all posts
    LIKES.with(|likes| {
        let mut likes = likes.borrow_mut();
        likes.insert("post-1".to_string(), Nat::from(0u32));
        likes.insert("post-2".to_string(), Nat::from(0u32));
        likes.insert("post-3".to_string(), Nat::from(0u32));
    });
}

#[ic_cdk::query]
fn get_posts() -> Vec<Post> {
    POSTS.with(|posts| {
        let posts = posts.borrow();
        posts.values().cloned().collect()
    })
}

#[ic_cdk::query]
fn get_post(post_id: String) -> Option<Post> {
    if post_id.trim().is_empty() {
        return None;
    }
    
    POSTS.with(|posts| {
        let posts = posts.borrow();
        posts.get(&post_id).cloned()
    })
}

#[ic_cdk::update]
fn create_post(id: String, title: String, content: String) -> Result<Post, String> {
    if id.trim().is_empty() || title.trim().is_empty() || content.trim().is_empty() {
        return Err("Post ID, title, and content cannot be empty".to_string());
    }
    
    POSTS.with(|posts| {
        let mut posts = posts.borrow_mut();
        
        if posts.contains_key(&id) {
            return Err("Post with this ID already exists".to_string());
        }
        
        let new_post = Post {
            id: id.clone(),
            title,
            content,
            created_at: ic_cdk::api::time(),
        };
        
        posts.insert(id.clone(), new_post.clone());
        
        // Initialize likes for the new post
        LIKES.with(|likes| {
            let mut likes = likes.borrow_mut();
            likes.insert(id, Nat::from(0u32));
        });
        
        Ok(new_post)
    })
}

#[ic_cdk::update]
fn like(post_id: String) -> Result<Nat, String> {
    if post_id.trim().is_empty() {
        return Err("Post ID cannot be empty".to_string());
    }
    
    // Check if post exists
    let post_exists = POSTS.with(|posts| {
        let posts = posts.borrow();
        posts.contains_key(&post_id)
    });
    
    if !post_exists {
        return Err("Post not found".to_string());
    }
    
    LIKES.with(|likes| {
        let mut likes = likes.borrow_mut();
        let current_likes = likes.get(&post_id).cloned().unwrap_or_else(|| Nat::from(0u32));
        let new_likes = current_likes + Nat::from(1u32);
        
        likes.insert(post_id, new_likes.clone());
        Ok(new_likes)
    })
}

#[ic_cdk::query]
fn get_likes(post_id: String) -> Result<Nat, String> {
    if post_id.trim().is_empty() {
        return Err("Post ID cannot be empty".to_string());
    }
    
    LIKES.with(|likes| {
        let likes = likes.borrow();
        let count = likes.get(&post_id).cloned().unwrap_or_else(|| Nat::from(0u32));
        Ok(count)
    })
}

#[ic_cdk::query]
fn get_posts_with_likes() -> Vec<(Post, Nat)> {
    POSTS.with(|posts| {
        let posts = posts.borrow();
        let mut result = Vec::new();
        
        for post in posts.values() {
            let likes = LIKES.with(|likes| {
                let likes = likes.borrow();
                likes.get(&post.id).cloned().unwrap_or_else(|| Nat::from(0u32))
            });
            
            result.push((post.clone(), likes));
        }
        
        result
    })
}

// Enable Candid export
ic_cdk::export_candid!(); 