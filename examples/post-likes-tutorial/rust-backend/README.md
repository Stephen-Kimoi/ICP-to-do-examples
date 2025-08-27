# Rust Backend for Post Likes Tutorial

This is the Rust implementation of the post likes backend using the [ICP Rust Agent](https://internetcomputer.org/docs/building-apps/interact-with-canisters/agents/rust-agent).

## Features

- **HTTP API Server**: Axum web framework with async/await support
- **ICP Integration**: Uses DFINITY Rust agent for canister communication
- **Type Safety**: Strong typing with Rust's type system
- **Performance**: High-performance async server with proper error handling
- **CORS Support**: Built-in CORS middleware for web applications

## API Endpoints

- `GET /health` - Health check
- `GET /likes/{post_id}` - Get likes for a post
- `POST /like/{post_id}` - Like a post
- `GET /posts` - Get all posts with like counts
- `POST /posts` - Create a new post

## Quick Start

### Prerequisites

- Rust 1.70+ installed
- DFX running locally with the post-likes-backend canister deployed
- Environment variables configured

### Installation

```bash
cd rust-backend
cargo build --release
```

### Configuration

Set the following environment variables:

```bash
export PORT=3001
export DFX_NETWORK=local
export POST_LIKES_BACKEND_CANISTER_ID=your-canister-id
```

### Running the Server

```bash
cargo run --release
```

The server will start on port 3001 (or the port specified in your environment).

## Architecture

This backend demonstrates:

1. **Agent Initialization**: Setting up the ICP Rust agent with proper error handling
2. **Async/Await**: Full async support for high-performance canister communication
3. **Type Safety**: Strong typing with Rust's Result types and error handling
4. **Web Framework**: Modern web framework (Axum) with proper middleware
5. **CORS Support**: Built-in CORS handling for cross-origin requests

## Key Implementation Details

### Agent Setup

```rust
let agent = Agent::builder()
    .with_url(url)
    .build()?;

if !use_mainnet {
    agent.fetch_root_key().await?;
}
```

### Canister Communication

```rust
let canister = Canister::builder()
    .with_agent(&state.agent)
    .with_canister_id(state.canister_id)
    .build()?;

let result = canister.query("get_likes")
    .with_arg(Encode!(&post_id)?)
    .call()
    .await?;
```

### Error Handling

```rust
match result {
    Ok(likes) => Ok(Json(LikesResponse { ... })),
    Err(err) => {
        warn!("Failed to get likes: {}", err);
        Err(StatusCode::BAD_REQUEST)
    }
}
```

## Dependencies

- **ic-agent**: DFINITY's Rust agent for ICP communication
- **ic-utils**: High-level utilities for canister interaction
- **axum**: Modern async web framework
- **tokio**: Async runtime
- **candid**: Candid interface language support
- **serde**: Serialization/deserialization

## Performance Features

- **Async/Await**: Non-blocking I/O for all operations
- **Connection Pooling**: Efficient agent connection management
- **Error Recovery**: Proper error handling without performance impact
- **Memory Safety**: Rust's memory safety guarantees

## Security Considerations

- **Anonymous Identity**: Currently uses anonymous identity for simplicity
- **Input Validation**: All inputs are validated before processing
- **Error Messages**: Safe error messages that don't expose internals
- **CORS**: Configurable CORS for production deployments

## Testing

You can test the API endpoints using curl or any HTTP client:

```bash
# Health check
curl http://localhost:3001/health

# Get posts
curl http://localhost:3001/posts

# Like a post
curl -X POST http://localhost:3001/like/post-1
```

## Troubleshooting

- **Build Errors**: Ensure you have the latest Rust toolchain
- **Canister Connection**: Verify the canister ID and network configuration
- **Port Conflicts**: Check if port 3001 is available
- **Dependencies**: Run `cargo update` if you encounter dependency issues

## Next Steps

- Implement proper identity management
- Add authentication and authorization
- Implement rate limiting
- Add metrics and monitoring
- Deploy to production environments
