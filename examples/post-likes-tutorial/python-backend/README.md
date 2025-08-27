# Post Likes Python Backend - Code Implementation Guide

This guide explains how to build a Python Web2 API that integrates with an ICP canister using the ic-py library. We'll walk through the code structure and show you exactly how to call your backend canister from a Python service.

## 🏗️ Architecture Overview

The Python backend consists of a single `main.py` file that handles:
- FastAPI web server setup
- ICP Python agent (ic-py) integration
- HTTP endpoint handlers
- Canister communication

## 📁 Code Structure

### **Dependencies and Imports**

```python
import os
import asyncio
from typing import List, Optional
from fastapi import FastAPI, HTTPException, Path
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from dotenv import load_dotenv
from contextlib import asynccontextmanager
from ic.client import Client
from ic.identity import Identity
from ic.agent import Agent
from ic.canister import Canister
```
*Source: [main.py lines 1-15](main.py#L1-L15)*

Key imports:
- **`fastapi`**: Modern, fast web framework for building APIs
- **`ic-py`**: Python library for ICP integration (`ic.client`, `ic.identity`, `ic.agent`, `ic.canister`)
- **`pydantic`**: Data validation using Python type annotations
- **`asyncio`**: Asynchronous programming support

### **Environment Configuration**

```python
# Load environment variables
load_dotenv()
```
*Source: [main.py lines 17-18](main.py#L17-L18)*

Loads environment variables from a `.env` file for configuration.

### **FastAPI Application Setup**

```python
@asynccontextmanager
async def lifespan(app: FastAPI):
    """Lifespan context manager for FastAPI app"""
    # Startup
    await initialize_agent()
    yield
    # Shutdown
    pass

app = FastAPI(title="Post Likes Python Backend", version="1.0.0", lifespan=lifespan)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
```
*Source: [main.py lines 20-35](main.py#L20-L35)*

**Application Configuration:**
- **`lifespan`**: Context manager for startup/shutdown operations
- **`initialize_agent()`**: Called during startup to set up ICP connection
- **CORS Middleware**: Enables cross-origin requests for frontend integration

### **Data Models (Pydantic)**

```python
# Pydantic models
class Post(BaseModel):
    id: str
    title: str
    content: str
    created_at: int

class CreatePostRequest(BaseModel):
    id: str
    title: str
    content: str

class ApiResponse(BaseModel):
    data: Optional[Post] = None
    message: str
    error: Optional[str] = None

class LikesResponse(BaseModel):
    post_id: str
    likes: str
    message: str

class LikeResponse(BaseModel):
    post_id: str
    new_likes: str
    message: str

class PostsResponse(BaseModel):
    posts: List[Post]
    message: str
```
*Source: [main.py lines 37-65](main.py#L37-L65)*

**Pydantic Models:**
- **`Post`**: Represents a post from the canister
- **`CreatePostRequest`**: Input validation for creating posts
- **`ApiResponse`**: Generic response wrapper with optional data and error fields
- **Response Models**: Specific response structures for different endpoints

### **Global Variables and Agent Initialization**

```python
# Global variables for agent and canister
agent: Optional[Agent] = None
canister: Optional[Canister] = None
```
*Source: [main.py lines 67-68](main.py#L67-L68)*

Global variables store the initialized ICP agent and canister instances for use across all endpoints.

#### **Agent Initialization Function**

```python
async def initialize_agent():
    """Initialize the ICP agent and canister connection"""
    global agent, canister
    
    try:
        # Get configuration from environment
        dfx_network = os.getenv("DFX_NETWORK", "local")
        canister_id = os.getenv("POST_LIKES_BACKEND_CANISTER_ID")
        
        if not canister_id:
            raise ValueError("POST_LIKES_BACKEND_CANISTER_ID environment variable must be set")
        
        # Create identity (anonymous for this example)
        identity = Identity()
        
        # Create client
        if dfx_network == "local":
            client = Client("http://127.0.0.1:4943")
        else:
            client = Client("https://ic0.app")
        
        # Create agent
        agent = Agent(identity, client)
        
        # Read Candid interface from the generated .did file
        candid_file_path = "../src/declarations/post_likes_backend/post_likes_backend.did"
        try:
            with open(candid_file_path, 'r') as f:
                candid_interface = f.read()
        except FileNotFoundError:
            raise FileNotFoundError(f"Candid interface file not found at {candid_file_path}. Make sure to run 'dfx generate' first.")
        
        canister = Canister(
            agent=agent,
            canister_id=canister_id,
            candid=candid_interface
        )
        
        print(f"Python backend initialized with canister ID: {canister_id}")
        
    except Exception as e:
        print(f"Failed to initialize agent: {e}")
        raise
```
*Source: [main.py lines 70-105](main.py#L70-L105)*

**Initialization Process:**
1. **Environment Variables**: Reads network configuration and canister ID
2. **Identity Creation**: Creates anonymous identity for canister access
3. **Client Setup**: Configures client for local development or mainnet
4. **Agent Creation**: Initializes the ICP agent with identity and client
5. **Candid Interface**: Reads the generated `.did` file for type information
6. **Canister Instance**: Creates canister object for method calls

### **API Endpoints**

#### **Health Check Endpoint**

```python
@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return ApiResponse(
        message="Python backend is running and connected to ICP"
    )
```
*Source: [main.py lines 108-112](main.py#L108-L112)*

Simple health check to verify the service is running and connected to ICP.

#### **Get Likes Endpoint**

```python
@app.get("/likes/{post_id}")
async def get_likes(post_id: str = Path(..., description="The ID of the post")):
    """Get likes for a specific post"""
    if not post_id or post_id.strip() == "":
        raise HTTPException(status_code=400, detail="Post ID cannot be empty")
    
    try:
        # Call the canister method
        result = await canister.get_likes_async(post_id)
        
        # Return raw result as-is
        return {
            "post_id": post_id,
            "result": result,
            "message": "Likes retrieved from ICP canister"
        }
            
    except Exception as e:
        print(f"Error getting likes: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to get likes: {str(e)}")
```
*Source: [main.py lines 114-130](main.py#L114-L130)*

**Get Likes Process:**
1. **Path Parameter**: Extracts `post_id` from URL path
2. **Input Validation**: Ensures post_id is not empty
3. **Canister Call**: `canister.get_likes_async(post_id)` calls the ICP method
4. **Response Format**: Returns raw result from canister with metadata

#### **Like Post Endpoint**

```python
@app.post("/like/{post_id}")
async def like_post(post_id: str = Path(..., description="The ID of the post to like")):
    """Like a specific post"""
    if not post_id or post_id.strip() == "":
        raise HTTPException(status_code=400, detail="Post ID cannot be empty")
    
    try:
        # Call the canister method
        result = await canister.like_async(post_id)
        
        # Return raw result as-is
        return {
            "post_id": post_id,
            "result": result,
            "message": "Post liked successfully on ICP canister"
        }
            
    except Exception as e:
        print(f"Error liking post: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to like post: {str(e)}")
```
*Source: [main.py lines 132-148](main.py#L132-L148)*

**Like Post Process:**
1. **Path Parameter**: Gets `post_id` from URL path
2. **Input Validation**: Checks post_id is not empty
3. **State Update**: `canister.like_async(post_id)` modifies canister state
4. **Response Handling**: Returns the result with success message

#### **Get Posts Endpoint**

```python
@app.get("/posts")
async def get_posts():
    """Get all posts with their like counts"""
    try:
        # Call the canister method
        result = await canister.get_posts_with_likes_async()
        
        # Just return the raw result as-is, like the Node.js backend does
        return {
            "posts": result,
            "message": "Posts retrieved directly from ICP canister"
        }
            
    except Exception as e:
        print(f"Error getting posts: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to get posts: {str(e)}")
```
*Source: [main.py lines 150-165](main.py#L150-L165)*

**Get Posts Process:**
1. **Canister Query**: `canister.get_posts_with_likes_async()` fetches all posts
2. **Raw Response**: Returns the unmodified result from canister
3. **Error Handling**: Catches and formats any exceptions

#### **Create Post Endpoint**

```python
@app.post("/posts")
async def create_post(request: CreatePostRequest):
    """Create a new post"""
    if not request.id or not request.title or not request.content:
        raise HTTPException(status_code=400, detail="All fields are required")
    
    try:
        # Call the canister method
        result = await canister.create_post_async(request.id, request.title, request.content)
        
        # Return raw result as-is
        return {
            "result": result,
            "message": "Post created successfully on ICP canister"
        }
            
    except Exception as e:
        print(f"Error creating post: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to create post: {str(e)}")
```
*Source: [main.py lines 167-183](main.py#L167-L183)*

**Create Post Process:**
1. **Request Validation**: Pydantic automatically validates the request body
2. **Field Validation**: Manual check for required fields
3. **Canister Update**: `canister.create_post_async()` creates new post
4. **Response Format**: Returns the created post result

### **Application Entry Point**

```python
if __name__ == "__main__":
    import uvicorn
    
    port = int(os.getenv("PORT", "3002"))
    
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=port,
        reload=True
    )
```
*Source: [main.py lines 185-215](main.py#L185-L215)*

**Server Configuration:**
- **Port**: Configurable via environment variable, defaults to 3002
- **Host**: Binds to all interfaces (0.0.0.0)
- **Reload**: Enables auto-reload for development

## 🔑 Key Concepts for ICP Integration

### **Async/Await Pattern**
All canister calls use `async/await` for non-blocking operations:
```python
# Example of async canister call
result = await canister.method_name_async(arguments)
```

### **Canister Method Naming**
ic-py automatically appends `_async` to method names:
- `get_likes()` → `get_likes_async()`
- `create_post()` → `create_post_async()`
- `like()` → `like_async()`

### **Error Handling Strategy**
1. **Input Validation**: Check parameters before canister calls
2. **Exception Catching**: Wrap canister calls in try-catch blocks
3. **HTTP Status Codes**: Use appropriate status codes for different error types
4. **Error Messages**: Provide clear error details for debugging

### **Candid Interface Integration**
- **`.did` File**: Contains the canister's interface definition
- **Type Safety**: Pydantic models ensure data validation
- **Raw Results**: Return canister responses directly for flexibility

## Running the Service

### Development Mode
```bash
python main.py
```

### Production Mode
```bash
uvicorn main:app --host 0.0.0.0 --port 3002
```

## 🔧 Configuration

Create a `.env` file with:
```env
PORT=3002
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=your-canister-id
```

## 📚 Next Steps

After understanding this implementation:
1. **Add authentication middleware** using FastAPI's security features
2. **Implement request validation** with Pydantic validators
3. **Add logging and monitoring** for production deployment
4. **Implement caching** for frequently accessed data
5. **Add rate limiting** to prevent abuse
6. **Set up proper error handling** with custom exception classes

## 🔍 Key Differences from Other Implementations

### **vs Node.js Backend**
- **Async Methods**: Python uses `_async` suffix, Node.js doesn't
- **Error Handling**: Python uses FastAPI exceptions, Node.js uses Express error handling
- **Type Validation**: Python uses Pydantic, Node.js uses manual validation

### **vs Rust Backend**
- **Performance**: Python is slower but easier to develop
- **Type Safety**: Rust provides compile-time guarantees, Python uses runtime validation
- **Memory Management**: Rust has manual memory management, Python uses garbage collection

This implementation demonstrates how to leverage Python's simplicity and FastAPI's modern features to create a robust Web2 API that integrates seamlessly with ICP canisters, making it an excellent choice for rapid prototyping and data science applications.
