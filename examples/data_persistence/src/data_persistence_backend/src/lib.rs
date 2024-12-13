//! # Data Persistence Example Using Stable Memory
//! This module demonstrates how to implement data persistence across canister upgrades
//! using VirtualMemory and DefaultMemoryImpl from ic-stable-structures.

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use serde::{Deserialize, Serialize};

/// Type alias for our memory implementation using VirtualMemory with DefaultMemoryImpl
type Memory1 = VirtualMemory<DefaultMemoryImpl>;

/// Represents a single note in the system
#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
struct Note {
    /// Unique identifier for the note
    id: u64,
    /// Title of the note
    title: String,
    /// Main content of the note
    content: String,
    /// Timestamp of when the note was created
    created_at: u64,
}

/// Implementation of Storable trait for Note to enable stable storage
impl Storable for Note {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Implementation of BoundedStorable for Note to set size limits
impl BoundedStorable for Note {
    const MAX_SIZE: u32 = 1024; // Maximum size in bytes for a single note
    const IS_FIXED_SIZE: bool = false;
}

/// Thread-local storage for our stable memory structures
thread_local! {
    /// Memory manager using DefaultMemoryImpl
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    /// Stable B-tree map for storing notes using VirtualMemory
    static NOTE_MAP: RefCell<StableBTreeMap<u64, Note, Memory1>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    /// Counter for generating unique note IDs
    static COUNTER: RefCell<Cell<u64, Memory1>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            0
        ).unwrap()
    );
}

/// Creates a new note
/// 
/// # Arguments
/// * `title` - The title of the note
/// * `content` - The content of the note
/// 
/// # Returns
/// The newly created note
#[ic_cdk::update]
fn create_note(title: String, content: String) -> Note {
    let id = COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value
    });

    let note = Note {
        id,
        title,
        content,
        created_at: ic_cdk::api::time(),
    };

    NOTE_MAP.with(|notes| {
        notes.borrow_mut().insert(id, note.clone());
    });

    note
}

/// Retrieves a note by its ID
/// 
/// # Arguments
/// * `id` - The ID of the note to retrieve
/// 
/// # Returns
/// Option containing the note if found
#[ic_cdk::query]
fn get_note(id: u64) -> Option<Note> {
    NOTE_MAP.with(|notes| notes.borrow().get(&id))
}

/// Updates an existing note
/// 
/// # Arguments
/// * `id` - The ID of the note to update
/// * `title` - The new title
/// * `content` - The new content
/// 
/// # Returns
/// Option containing the updated note if found
#[ic_cdk::update]
fn update_note(id: u64, title: String, content: String) -> Option<Note> {
    NOTE_MAP.with(|notes| {
        let mut notes = notes.borrow_mut();
        if let Some(mut note) = notes.get(&id) {
            note.title = title;
            note.content = content;
            notes.insert(id, note.clone());
            Some(note)
        } else {
            None
        }
    })
}

/// Deletes a note by its ID
/// 
/// # Arguments
/// * `id` - The ID of the note to delete
/// 
/// # Returns
/// Boolean indicating whether the note was successfully deleted
#[ic_cdk::update]
fn delete_note(id: u64) -> bool {
    NOTE_MAP.with(|notes| notes.borrow_mut().remove(&id).is_some())
}

/// Retrieves all notes
/// 
/// # Returns
/// Vector containing all notes
#[ic_cdk::query]
fn list_notes() -> Vec<Note> {
    NOTE_MAP.with(|notes| {
        notes.borrow()
            .iter()
            .map(|(_, note)| note.clone())
            .collect()
    })
}

// Required for Candid interface generation
ic_cdk::export_candid!();