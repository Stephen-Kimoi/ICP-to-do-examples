# Data Persistence Using Stable Memory in Canisters

This project demonstrates how to implement **data persistence across canister upgrades** using the **[ic-stable-memory](https://github.com/seniorjoinu/ic-stable-memory)** library. 

Stable storage ensures that critical application data is not lost during canister upgrades, making it essential for stateful applications.

This example walks you through a simple **Note-taking application** backend, where data like notes and counters are stored persistently.

---

## **Overview**

The application uses the following stable structures:

- **VirtualMemory**: Abstracts stable memory to allow efficient management of memory allocation and access.
- **DefaultMemoryImpl**: Provides the default implementation of stable memory.
- **StableBTreeMap**: A persistent, ordered key-value store for storing notes.
- **Cell**: A simple, persistent storage structure for a single value, used here to maintain a unique counter for note IDs.

---

## **Core Features**

1. **Create Note**  
   Adds a new note with a unique ID, title, content, and timestamp.  
   Data is stored in a stable B-tree map, ensuring persistence across upgrades.

2. **Retrieve Note**  
   Fetches a specific note by its ID.

3. **Update Note**  
   Modifies the title or content of an existing note, identified by its ID.

4. **Delete Note**  
   Removes a note from the storage by its ID.

5. **List All Notes**  
   Retrieves all notes stored in the application.

---

## **How the Code Works**

### **Stable Storage Initialization**

Stable memory structures are initialized using the `thread_local!` macro to ensure thread safety:

- **Memory Manager**: Manages access to stable memory.
- **Stable B-tree Map**: Stores notes with `id` as the key and `Note` as the value.
- **Counter Cell**: Maintains a persistent counter for generating unique note IDs.

Example [code snippet](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L44):
```rust
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
```

---
 
## **Working with Notes** 

####  **Creating a Note**
- Increments the counter to generate a unique id.
- Adds the new note to the stable B-tree map.

Example [code snippet](hhttps://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L76):
```rust 
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
```

#### **Retrieving a Note**
Fetches a note by its unique id:

Example [code snippet](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L105):
```rust
#[ic_cdk::query]
fn get_note(id: u64) -> Option<Note> {
    NOTE_MAP.with(|notes| notes.borrow().get(&id))
}
``` 

#### **Updating a Note**

Updates the title and content of an existing note:

Example [code snippet](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L119):
```rust
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
```

#### **Deleting a Note** 

Deletes a note by removing it from the stable B-tree map:

Example [code snippet](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L119):
```rust
#[ic_cdk::update]
fn delete_note(id: u64) -> bool {
    NOTE_MAP.with(|notes| notes.borrow_mut().remove(&id).is_some())
}
```

#### **Listing All Notes**

Retrieves all notes in the system:

Example [code snippet](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L150):
```rust
#[ic_cdk::query]
fn list_notes() -> Vec<Note> {
    NOTE_MAP.with(|notes| {
        notes.borrow()
            .iter()
            .map(|(_, note)| note.clone())
            .collect()
    })
}
```

### **Key Concepts**

1. **Stable Memory**

   Stable memory persists data across canister upgrades. This example uses the `ic-stable-memory` crate, which provides abstractions like `VirtualMemory` and `StableBTreeMap` to simplify working with stable memory.

2. **Thread Safety**

   Using `thread_local!` ensures each thread has its own instance of the memory structures, avoiding race conditions.

3. **Storable and BoundedStorable Traits**

   These traits are implemented for the `Note` struct to enable serialization and enforce size limits for stable storage.

Example [code snippet](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/4f3eb4343e69268bcbbab52f4fc392ac226d7f30/examples/data_persistence/src/data_persistence_backend/src/lib.rs#L28):
```rust
   impl Storable for Note {
       fn to_bytes(&self) -> Cow<[u8]> {
           Cow::Owned(Encode!(self).unwrap())
       }

       fn from_bytes(bytes: Cow<[u8]>) -> Self {
           Decode!(bytes.as_ref(), Self).unwrap()
       }
   }

   impl BoundedStorable for Note {
       const MAX_SIZE: u32 = 1024; // Maximum size in bytes for a single note
       const IS_FIXED_SIZE: bool = false;
   }
```

### **Upgrading the Canister Safely**

To ensure data persistence across canister upgrades, the following hooks are implemented:

---

#### **Saving State Before Upgrade**

The `pre_upgrade` hook saves the current state of the canister (notes and counter value) to stable memory. This ensures that all data is serialized and stored persistently before the upgrade process begins.

```rust
/// Save stable memory state before the canister is upgraded
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let notes: Vec<(u64, Note)> = NOTE_MAP.with(|notes| {
        notes
            .borrow()
            .iter()
            .map(|(key, value)| (key, value.clone()))
            .collect()
    });
    let counter_value = COUNTER.with(|counter| *counter.borrow().get());

    storage::stable_save((notes, counter_value)).expect("Failed to save stable state");
}
``` 

### **Key Points**

- The notes from the `NOTE_MAP` are retrieved and serialized as a vector of key-value pairs `(u64, Note)`.
- The current counter value is fetched from `COUNTER`.
- Both the notes and counter value are saved using `storage::stable_save`.

---

### **Restoring State After Upgrade**

The `post_upgrade` hook restores the saved state from stable memory after the canister has been upgraded. This ensures that the application resumes with the same data it had before the upgrade.

```rust
/// Restore stable memory state after the canister is upgraded
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    match storage::stable_restore::<(Vec<(u64, Note)>, u64)>() {
        Ok((notes, counter_value)) => {
            NOTE_MAP.with(|map| {
                let mut map = map.borrow_mut();
                for (key, value) in notes {
                    map.insert(key, value);
                }
            });

            COUNTER.with(|counter| {
                counter.borrow_mut().set(counter_value).expect("Failed to restore counter");
            });
        }
        Err(e) => {
            ic_cdk::println!("Failed to restore stable state: {:?}", e);
        }
    }
}
``` 

### **Key Points**

- The previously saved state (notes and counter value) is restored using `storage::stable_restore`.
- The notes are reinserted into the `NOTE_MAP`, recreating the original storage state.
- The counter value is restored into `COUNTER` to ensure note IDs remain consistent.
- Errors during restoration are logged to assist debugging if the state cannot be restored.
---

### **Performing the canister upgrade**

Once you've done this you can perform the canister upgrade by running the [following commands](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/main/CHEATSHEAT.md#updating-content-security-policy-csp)



