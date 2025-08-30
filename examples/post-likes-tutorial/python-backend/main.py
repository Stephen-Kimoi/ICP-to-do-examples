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

# Load environment variables
load_dotenv()

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

# Global variables for agent and canister
agent: Optional[Agent] = None
canister: Optional[Canister] = None

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



@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return ApiResponse(
        message="Python backend is running and connected to ICP"
    )

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

if __name__ == "__main__":
    import uvicorn
    
    port = int(os.getenv("PORT", "3002"))
    
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=port,
        reload=True
    )
