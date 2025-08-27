# Python Backend for Post Likes Tutorial

This is the Python implementation of the post likes backend using the [ICP Python Agent (ic-py)](https://github.com/eliezhao/ic-py/tree/fix/issues).

## Features

- **HTTP API Server**: FastAPI with automatic OpenAPI documentation
- **ICP Integration**: Uses ic-py Python agent for canister communication
- **Async Support**: Full async/await support for high-performance operations
- **Type Hints**: Comprehensive type hints for better development experience
- **Auto-documentation**: Automatic API documentation with Swagger UI

## API Endpoints

- `GET /health` - Health check
- `GET /likes/{post_id}` - Get likes for a post
- `POST /like/{post_id}` - Like a post
- `GET /posts` - Get all posts with like counts
- `POST /posts` - Create a new post

## Quick Start

### Prerequisites

- Python 3.8+ installed
- DFX running locally with the post-likes-backend canister deployed
- Environment variables configured

### Installation

```bash
cd python-backend
python3 -m venv venv 
source venv/bin/activate  
pip install -r requirements.txt
```

### Configuration

Create a `.env` file in the `python-backend` directory:

```env
PORT=3002
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=your-canister-id
```

### Running the Server

```bash
python main.py
```

Or using uvicorn directly:

```bash
uvicorn main:app --host 0.0.0.0 --port 3002 --reload
```

The server will start on port 3002 (or the port specified in your .env file).

## API Documentation

Once the server is running, you can access:

- **Swagger UI**: `http://localhost:3002/docs`
- **ReDoc**: `http://localhost:3002/redoc`
- **OpenAPI JSON**: `http://localhost:3002/openapi.json`

## Architecture

This backend demonstrates:

1. **Agent Initialization**: Setting up the ICP Python agent with proper error handling
2. **Async/Await**: Full async support for non-blocking canister communication
3. **FastAPI Integration**: Modern web framework with automatic validation
4. **Type Safety**: Comprehensive type hints and Pydantic models
5. **Error Handling**: Proper error handling with HTTP status codes

## Key Implementation Details

### Agent Setup

```python
identity = Identity()
client = Client("http://127.0.0.1:4943")
agent = Agent(identity, client)
```

### Canister Communication

```python
canister = Canister(
    agent=agent,
    canister_id=canister_id,
    candid=""
)

result = await canister.get_likes_async(post_id)
```

### Pydantic Models

```python
class Post(BaseModel):
    id: str
    title: str
    content: str
    created_at: int

class CreatePostRequest(BaseModel):
    id: str
    title: str
    content: str
```

## Dependencies

- **fastapi**: Modern, fast web framework for building APIs
- **uvicorn**: ASGI server for running FastAPI applications
- **ic-py**: Python agent for ICP communication
- **python-dotenv**: Environment variable management
- **pydantic**: Data validation using Python type annotations
- **httpx**: Async HTTP client

## Performance Features

- **Async/Await**: Non-blocking I/O for all operations
- **FastAPI**: High-performance web framework built on Starlette
- **Pydantic**: Fast data validation and serialization
- **Uvicorn**: High-performance ASGI server

## Security Considerations

- **Anonymous Identity**: Currently uses anonymous identity for simplicity
- **Input Validation**: Automatic validation through Pydantic models
- **CORS**: Configurable CORS middleware
- **Error Messages**: Safe error messages that don't expose internals

## Testing

You can test the API endpoints using curl, the Swagger UI, or any HTTP client:

```bash
# Health check
curl http://localhost:3002/health

# Get posts
curl http://localhost:3002/posts

# Like a post
curl -X POST http://localhost:3002/like/post-1

# Create a post
curl -X POST http://localhost:3002/posts \
  -H "Content-Type: application/json" \
  -d '{"id": "post-5", "title": "Test Post", "content": "Test content"}'
```

## Development Features

- **Hot Reload**: Automatic server restart on code changes
- **Type Checking**: Full type hint support for better development experience
- **Auto-completion**: IDE support for better development workflow
- **Documentation**: Automatic API documentation generation

## Troubleshooting

- **Import Errors**: Ensure all dependencies are installed with `pip install -r requirements.txt`
- **Canister Connection**: Verify the canister ID and network configuration
- **Port Conflicts**: Check if port 3002 is available
- **Python Version**: Ensure you're using Python 3.8 or higher

## Next Steps

- Implement proper identity management
- Add authentication and authorization
- Implement rate limiting
- Add metrics and monitoring
- Deploy to production environments
- Add comprehensive testing with pytest
- Implement database caching layer

## Comparison with Other Backends

| Feature | Node.js | Rust | Python |
|---------|---------|------|--------|
| **Performance** | Good | Excellent | Good |
| **Type Safety** | Basic | Excellent | Good |
| **Development Speed** | Fast | Medium | Fast |
| **Memory Usage** | Medium | Low | Medium |
| **Learning Curve** | Low | High | Low |
| **Ecosystem** | Large | Growing | Large |
