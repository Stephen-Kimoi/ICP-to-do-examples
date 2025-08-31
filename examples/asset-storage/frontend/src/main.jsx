import React, { useState, useRef, useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { backend } from 'declarations/backend';
import JSZip from 'jszip';
import mammoth from 'mammoth';
import '/index.css';

const App = () => {
  const [images, setImages] = useState([]);
  const [selectedFile, setSelectedFile] = useState(null);
  const [selectedImage, setSelectedImage] = useState(null);
  const [fileId, setFileId] = useState(null);
  const [isLoadingImage, setIsLoadingImage] = useState(false);
  const [isLoadingFile, setIsLoadingFile] = useState(false);
  const [files, setFiles] = useState([]);
  const [selectedFileContent, setSelectedFileContent] = useState(null);
  const [showPopup, setShowPopup] = useState(false);

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
        const id = await backend.upload_image([chunks]);
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

  const handleFileUpload = async (event) => {
    event.preventDefault();
    if (!selectedFile) return;

    setIsLoadingFile(true);
    const file = selectedFile.target.files[0];
    const reader = new FileReader();
    
    reader.onload = async () => {
      try {
        const fileData = new Uint8Array(reader.result);
        const id = await backend.upload_file(Array.from(fileData));
        setFileId(id);
        setSelectedFile(null);
      } catch (error) {
        console.error('Error uploading file:', error);
      } finally {
        setIsLoadingFile(false);
      }
    };
    
    reader.readAsArrayBuffer(file);
  };

  const loadImages = async () => {
    const allImages = await backend.get_all_images();
    setImages(allImages);
  };

  const loadFiles = async () => {
    const allFiles = await backend.get_all_files();
    console.log('All files:', allFiles);
    setFiles(allFiles);
  };

  const getFile = async (id) => {
    const file = await backend.get_file(id);
    if (file) {
      const blob = new Blob([new Uint8Array(file.data)]);
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `file-${id}`;
      a.click();
    }
  };

  const handleFileClick = async (file) => {
    console.log('File clicked:', file);

    // Create a Blob from the file data
    const blob = new Blob([new Uint8Array(file.data)]);
    console.log('File data:', file.data);

    // Check if the file is a ZIP archive (e.g., .docx)
    const isZipFile = file.data[0] === 0x50 && file.data[1] === 0x4B; // 'PK' signature

    if (isZipFile) {
        try {
            const zip = await JSZip.loadAsync(blob);
            const docContent = zip.file("word/document.xml");
            if (docContent) {
                const text = await docContent.async("text");
                console.log('Extracted Text:', text);

                // Optionally parse .docx content into readable text
                const readableText = await mammoth.extractRawText({ arrayBuffer: blob.arrayBuffer() });
                console.log('Readable Text:', readableText.value);
                setSelectedFileContent(readableText.value);
                setShowPopup(true);
            } else {
                console.log("The document.xml file was not found in the ZIP archive.");
            }
        } catch (error) {
            console.error("Error parsing ZIP file:", error);
        }
    } else {
        console.log("File is not a ZIP archive.");
        // Fallback for other file types
        const reader = new FileReader();
        reader.onload = () => {
            console.log('File Content:', reader.result);
        };
        reader.readAsText(blob);
    }
  };

  useEffect(() => {
    loadImages();
    loadFiles();
  }, []);

  return (
    <div className="container">
      <div className="left-section">
        <h2 className="section-title">Image Gallery</h2>
        <div className="upload-form">
          <form onSubmit={handleImageUpload}>
            <div className="form-group">
              <input 
                type="file" 
                accept="image/*" 
                onChange={setSelectedImage}
                className="input-file"
              />
              <button 
                type="submit" 
                className="button"
                disabled={isLoadingImage || !selectedImage}
              >
                {isLoadingImage ? 'Uploading...' : 'Upload Image'}
              </button>
            </div>
          </form>
        </div>

        <div className="image-grid">
          {images.length > 0 ? (
            images.map((image) => (
              <div key={image.id} className="image-container">
                <img 
                  src={URL.createObjectURL(new Blob([new Uint8Array(image.data[0])]))}
                  alt={`Upload ${image.id}`}
                  className="image"
                />
              </div>
            ))
          ) : (
            <p style={{ textAlign: 'center', color: 'var(--secondary-color)' }}>
              No images uploaded yet.
            </p>
          )}
        </div>

      </div>

      {/* <div className="right-section">
        <h2 className="section-title">File Management</h2>
        <div className="upload-form">
          <form onSubmit={handleFileUpload}>
            <div className="form-group">
              <input 
                type="file" 
                onChange={setSelectedFile}
                className="input-file"
              />
              <button 
                type="submit" 
                className="button"
                disabled={isLoadingFile || !selectedFile}
              >
                {isLoadingFile ? 'Uploading...' : 'Upload File'}
              </button>
            </div>
          </form>

          <div className="file-list">
            {files.map((file) => (
              <div 
                key={file.id} 
                className="file-item"
                onClick={() => handleFileClick(file)}
              >
                <span>File #{file.id}</span>
                <button className="view-button">View Content</button>
              </div>
            ))}
          </div>
          
          <div className="file-list">
            {fileId ? (
              <button 
                onClick={() => getFile(fileId)}
                className="button download-button"
              >
                Download File #{fileId}
              </button>
            ) : (
              <p style={{ textAlign: 'center', color: 'var(--secondary-color)' }}>
                No file uploaded yet.
              </p>
            )}
          </div>

        </div>

        {showPopup && (
          <div className="popup-overlay">
            <div className="popup-content">
              <button 
                className="close-button"
                onClick={() => setShowPopup(false)}
              >
                ×
              </button>
              <pre>{selectedFileContent}</pre>
            </div>
          </div>
        )}

      </div> */}
    </div>
  );
};

export default App;

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
