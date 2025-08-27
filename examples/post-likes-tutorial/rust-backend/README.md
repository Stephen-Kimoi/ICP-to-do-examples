# Post Likes Rust Backend

This is a Rust backend service that demonstrates how to use the ICP Rust agent to interact with a deployed post-likes canister on the Internet Computer.

## Features

- **ICP Integration**: Uses the official ICP Rust agent (`ic-agent`) to communicate with canisters
- **RESTful API**: Provides HTTP endpoints for all post-likes operations
- **Async/Await**: Built with Tokio for high-performance async operations
- **Error Handling**: Comprehensive error handling with detailed logging
- **CORS Support**: Cross-origin resource sharing enabled for frontend integration

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check endpoint |
| `GET` | `/posts` | Get all posts |
| `GET` | `/posts/:post_id` | Get a specific post by ID |
| `POST` | `/posts` | Create a new post |
| `GET` | `/likes/:post_id` | Get like count for a specific post |
| `POST` | `/like/:post_id` | Like a post (increment like count) |
| `GET` | `/posts-with-likes` | Get all posts with their like counts |

## Prerequisites

- Rust 1.70+ and Cargo
- dfx (DFINITY Canister SDK)
- A deployed post-likes backend canister

## Setup

1. **Clone and navigate to the project**:
   ```bash
   cd examples/post-likes-tutorial/rust-backend
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Set up environment variables**:
   Create a `.env` file in the project root with the following variables:
   ```bash
   # ICP Network Configuration
   # Set to "local" for local development, "ic" for mainnet
   DFX_NETWORK=local
   
   # Post Likes Backend Canister ID
   # For local development, this will be generated when you deploy the canister
   # For mainnet, use the deployed canister ID
   POST_LIKES_BACKEND_CANISTER_ID=bkyz2-fmaaa-aaaaa-qaaaq-cai
   
   # Optional: Custom ICP Agent URL (defaults to ic0.app for mainnet, 127.0.0.1:4943 for local)
   # ICP_AGENT_URL=https://ic0.app
   ```

4. **Deploy the post-likes backend canister** (if not already deployed):
   ```bash
   cd ../../
   dfx deploy post_likes_backend
   ```

5. **Update the canister ID** in your `.env` file with the deployed canister ID from the previous step.

## Running the Service

### Development Mode
```bash
cargo run
```

The service will start on `http://localhost:3000`.

### Production Build
```bash
cargo build --release
./target/release/post-likes-rust-backend
```

## Testing the API

### Health Check
```bash
curl http://localhost:3000/health
```

### Get All Posts
```bash
curl http://localhost:3000/posts
```

### Create a New Post
```bash
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{
    "id": "my-post-1",
    "title": "My First Post",
    "content": "This is the content of my first post."
  }'
```

### Get a Specific Post
```bash
curl http://localhost:3000/posts/my-post-1
```

### Get Likes for a Post
```bash
curl http://localhost:3000/likes/my-post-1
```

### Like a Post
```bash
curl -X POST http://localhost:3000/like/my-post-1
```

### Get Posts with Likes
```bash
curl http://localhost:3000/posts-with-likes
```

## Architecture

The application is structured as follows:

- **`main.rs`**: Main application entry point with Axum web server setup
- **`post_likes_client.rs`**: ICP agent client for interacting with the post-likes canister
- **Data Structures**: Candid-compatible structs matching the canister interface

### Key Components

1. **ICP Agent**: Handles communication with the Internet Computer
2. **PostLikesClient**: High-level client for canister operations
3. **Axum Router**: HTTP routing and request handling
4. **Error Handling**: Comprehensive error handling with detailed logging

## Error Handling

The service provides detailed error messages and logging for debugging:

- **Canister Errors**: Errors returned from the ICP canister
- **Network Errors**: Connection issues with the Internet Computer
- **Validation Errors**: Invalid input data
- **Encoding/Decoding Errors**: Candid serialization issues

## Logging

The service uses the `tracing` crate for structured logging:

- **Request Logging**: All incoming requests are logged
- **Debug Information**: Detailed debug information for canister operations
- **Error Logging**: Comprehensive error logging with context

## Development

### Adding New Endpoints

1. Add the route in `main.rs`
2. Implement the handler function
3. Add any necessary data structures
4. Update the client if needed

### Testing

Run the test suite:
```bash
cargo test
```

### Code Formatting

Format the code:
```bash
cargo fmt
```

### Linting

Check for code issues:
```bash
cargo clippy
```

## Troubleshooting

### Common Issues

1. **Canister ID Not Found**: Ensure the canister is deployed and the ID is correct
2. **Network Connection Issues**: Check your dfx network configuration
3. **Permission Denied**: Ensure the canister has the necessary permissions

### Debug Mode

Enable debug logging by setting the `RUST_LOG` environment variable:
```bash
RUST_LOG=debug cargo run
```

## Comparison with Other Implementations

This Rust implementation provides:

- **Performance**: Rust's zero-cost abstractions and async runtime
- **Type Safety**: Strong type system with compile-time guarantees
- **Memory Safety**: No garbage collection overhead
- **Native ICP Integration**: Direct use of the official Rust agent

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is part of the ICP Tutorial Examples and follows the same licensing terms.
