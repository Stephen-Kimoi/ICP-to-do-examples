# Image and File Storage Tutorial for Internet Computer

This tutorial will guide you through the process of implementing image and file storage on the Internet Computer. We'll cover the backend implementation in Rust and the frontend implementation in JavaScript.

```bash 
git clone https://github.com/Stephen-Kimoi/ICP-to-do-examples.git

cd ICP-to-do-examples/examples/asset-storage

dfx start --clean --background

npm run start
``` 

You can now interact with the application by uploading images and files.

[Image Upload Demo](https://youtu.be/qi8lL2qksX4)

[File Upload Demo](https://youtu.be/0_SWb_8KIn8)

## 1. Image Storage Implementation: 

### Backend (Rust): 
```rust 
// In lib.rs
#[derive(CandidType, Serialize, Deserialize, Clone)]
struct Image {
    id: u64,
    data: Vec<Vec<u8>>,
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
```

Function Breakdown: 
- ```Vec<Vec<u8>>```: Takes image data as nested vectors of bytes
- ```NEXT_ID.with```: Manages unique ID generation
- ```images.borrow_mut()```: Thread-safe access to stored images
- Returns: Generated ID for the uploaded image

### Frontend (React): 
```javascript
const handleImageUpload = async (event) => {
    event.preventDefault();
    if (!selectedImage) return;

    setIsLoadingImage(true);
    const file = selectedImage.target.files[0];
    const reader = new FileReader();
    
    reader.onload = async () => {
        try {
            const imageData = new Uint8Array(reader.result);
            const chunks = Array.from(imageData);
            const id = await asset_storage_backend.upload_image([chunks]);
            loadImages();
            setSelectedImage(null);
        } catch (error) {
            console.error('Error uploading image:', error);
        } finally {
            setIsLoadingImage(false);
        }
    };
    
    reader.readAsArrayBuffer(file);
};
```

Function Breakdown:
- Converts selected file to ```ArrayBuffer```
- Transforms data to ```Uint8Array```
- Sends to backend as ```array chunks```

## 2. File Storage Functions: 

### Backend (Rust):

```rust 
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
```
Function Breakdown:

- Takes ```raw file bytes as Vec```
- Generates unique file ID
- Stores in thread-safe HashMap
- Returns file identifier

### Frontend (React):

```javascript
const handleFileClick = async (file) => {
    const blob = new Blob([new Uint8Array(file.data)]);
    const reader = new FileReader();
    
    reader.onload = () => {
        setSelectedFileContent(reader.result);
        setShowPopup(true);
    };
    
    reader.readAsText(blob);
};
```

Function Breakdown:

- Converts ```binary data``` to ``Blob``
- Creates ``readable stream``

## Note:

``Maximum chunk size``:  the maximum chunk size for an upload (both image and file) is around 2MB. This is because Internet Computer limits their message size limit to 2MB. [Link](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/resource-limits)

Meaning if you're uploading larger files, you'll need to need to implement a chunking mechanism. Which involves splitting the file into smaller chunks and uploading each chunk separately.

