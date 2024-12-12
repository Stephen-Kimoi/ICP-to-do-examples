use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use serde::{Deserialize, Serialize};

// Define the type for our memory
type Memory1 = VirtualMemory<DefaultMemoryImpl>;

#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
struct Note {
    id: u64,
    title: String,
    content: String,
    created_at: u64,
}

// Implement Storable for Note
impl Storable for Note {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement BoundedStorable for Note
impl BoundedStorable for Note {
    const MAX_SIZE: u32 = 1024; // Maximum size in bytes
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static NOTE_MAP: RefCell<StableBTreeMap<u64, Note, Memory1>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static COUNTER: RefCell<Cell<u64, Memory1>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            0
        ).unwrap()
    );
}

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
#[ic_cdk::query]
fn get_note(id: u64) -> Option<Note> {
    NOTE_MAP.with(|notes| {
        notes.borrow().get(&id)
    })
}

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

#[ic_cdk::update]
fn delete_note(id: u64) -> bool {
    NOTE_MAP.with(|notes| {
        notes.borrow_mut().remove(&id).is_some()
    })
}

#[ic_cdk::query]
fn list_notes() -> Vec<Note> {
    NOTE_MAP.with(|notes| {
        notes.borrow()
            .iter()
            .map(|(_, note)| note.clone())
            .collect()
    })
}

ic_cdk::export_candid!();
