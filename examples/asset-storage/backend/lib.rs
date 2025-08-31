use std::collections::HashMap;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct Image {
    id: u64,
    data: Vec<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct File {
    id: u64,
    data: Vec<u8>,
}

thread_local! {
    static IMAGES: RefCell<HashMap<u64, Image>> = RefCell::new(HashMap::new());
    static FILES: RefCell<HashMap<u64, File>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);
}

#[ic_cdk::update]
fn upload_image(image_data: Vec<Vec<u8>>) -> u64 {
    NEXT_ID.with(|next_id| {
        let id = *next_id.borrow();
        next_id.borrow_mut().clone_from(&(id + 1));

        IMAGES.with(|images| {
            images.borrow_mut().insert(
                id,
                Image {
                    id,
                    data: image_data,
                }
            );
        });
        id
    })
}

#[ic_cdk::query]
fn get_all_images() -> Vec<Image> {
    IMAGES.with(|images| {
        images.borrow().values().cloned().collect()
    })
}

#[ic_cdk::update]
fn upload_file(file_data: Vec<u8>) -> u64 {
    NEXT_ID.with(|next_id| {
        let id = *next_id.borrow();
        next_id.borrow_mut().clone_from(&(id + 1));

        FILES.with(|files| {
            files.borrow_mut().insert(
                id,
                File {
                    id,
                    data: file_data,
                }
            );
        });
        id
    })
}

#[ic_cdk::query]
fn get_file(id: u64) -> Option<File> {
    FILES.with(|files| {
        files.borrow().get(&id).cloned()
    })
}

#[ic_cdk::query]
fn get_all_files() -> Vec<File> {
    FILES.with(|files| {
        files.borrow().values().cloned().collect()
    })
}

ic_cdk::export_candid!(); 