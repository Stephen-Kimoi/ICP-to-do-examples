use candid::Nat;
use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static LIKES: RefCell<HashMap<String, Nat>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
fn like(post_id: String) -> Result<Nat, String> {
    if post_id.trim().is_empty() {
        return Err("Post ID cannot be empty".to_string());
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

// Enable Candid export
ic_cdk::export_candid!(); 