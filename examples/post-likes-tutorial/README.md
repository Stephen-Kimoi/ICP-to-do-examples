# Tamper-Proof Post Likes: Web2 API + ICP Integration Tutorial

This tutorial demonstrates how to integrate your existing Web2 API withInternet Computer (ICP) to create tamper-proof, decentralized functionality. 

We'll build a simple likes counter system where your Web2 backend handles the API logic, but the actual like counts are stored immutably on ICP.

## What You'll Learn

- **Web2 + ICP Integration**: How to bridge traditional web APIs with blockchain functionality
- **ICP Agents**: Using the [JavaScript agent](https://internetcomputer.org/docs/building-apps/interact-with-canisters/agents/javascript-agent/) to call canisters from [Node.js](https://internetcomputer.org/docs/building-apps/interact-with-canisters/agents/nodejs/)
- **Tamper-Proof Data**: Why decentralization matters for data integrity

## Architecture Overview: 

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web2 API      │    │   ICP Agent     │    │   ICP Canister  │
│   (Node.js)     │───▶│   (JavaScript)  │───▶│   (Rust)        │
│                 │    │                 │    │                 │
│ • HTTP Endpoints│    │ • Authentication│    │ • Likes Storage │
│ • Business Logic│    │ • CBOR Encoding │    │ • Immutable     │
│ • Error Handling│    │ • Network Calls │    │ • Decentralized │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🛠️ What We're Building

### 1. **ICP Canister (Rust)**
- Stores posts and like counts for different posts
- `get_posts()` - Returns all posts (query call)
- `get_post(post_id)` - Returns a specific post (query call)
- `create_post(id, title, content)` - Creates a new post (update call)
- `like(post_id)` - Increments likes (update call)
- `get_likes(post_id)` - Retrieves likes (query call)
- `get_posts_with_likes()` - Returns posts with their like counts (query call)
- `initialize_sample_posts()` - Initializes sample posts (update call)
- Tamper-proof and decentralized storage

### 2. **Web2 API (Node.js + Express)**
- `GET /health` - Health check endpoint
- `GET /likes/:postId` - Get likes for a post
- `POST /like/:postId` - Like a post
- `GET /posts` - Get all posts with like counts
- `POST /posts` - Create a new post

### 3. **Integration Layer**
- DFINITY JavaScript agent for canister communication
- Server-side identity management
- Error handling and response processing

## Quick Start:

### Prerequisites
- [Environment Setup](../cheatsheet/environment-setup.md) - Complete setup guide for DFX, Node.js, and Rust
- Basic knowledge of Rust and Node.js

### Step 1: Clone and Setup
```bash
# Clone the repo 
git clone https://github.com/Stephen-Kimoi/ICP-to-do-examples

# Navigate to the project
cd examples/post-likes-tutorial

# Start dfx 
dfx start --clean --background

# Generate candid file, declarations and deploy the repo
generate-did post_likes_backend && dfx generate && dfx deploy

# Initialize sample posts in the canister
dfx canister call post_likes_backend initialize_sample_posts

# Install Web2 API dependencies
cd web2-api
npm install

# Run the server
npm start
```

## 📡 API Endpoints

### Health Check
```bash
GET /health
```
**Response:**
```json
{
  "status": "OK",
  "message": "Web2 API is running and connected to ICP"
}
```

### Get Likes for a Post
```bash
GET /likes/{postId}
```
**Response:**
```json
{
  "postId": "post-1",
  "likes": "5",
  "message": "Likes retrieved from ICP canister"
}
```

### Like a Post
```bash
POST /like/{postId}
```
**Response:**
```json
{
  "postId": "post-1",
  "newLikes": "6",
  "message": "Post liked successfully on ICP canister"
}
```

### Get All Posts with Likes
```bash
GET /posts
```
**Response:**
```json
{
  "posts": [
    {
      "id": "post-1",
      "title": "Getting Started with ICP",
      "content": "Learn the basics of Internet Computer Protocol and how to build decentralized applications.",
      "created_at": "1703123456789000000",
      "likes": "6"
    }
  ],
  "message": "Posts retrieved directly from ICP canister"
}
```

### Create a New Post
```bash
POST /posts
Content-Type: application/json

{
  "id": "post-4",
  "title": "My New Post",
  "content": "This is the content of my new post."
}
```
**Response:**
```json
{
  "post": {
    "id": "post-4",
    "title": "My New Post",
    "content": "This is the content of my new post.",
    "created_at": "1703123456789000000"
  },
  "message": "Post created successfully on ICP canister"
}
```

## 🔑 Key Concepts Explained

### **Why ICP for Likes?**
- **Tamper-Proof**: Once stored, likes cannot be manipulated by backend administrators
- **Decentralized**: Data is stored across multiple nodes, not controlled by a single entity
- **Verifiable**: Anyone can verify the authenticity of like counts
- **Cost-Effective**: ICP storage is significantly cheaper than traditional cloud storage

### **How the Agent Works**

The ICP JavaScript agent handles the communication between your Web2 API and the ICP canister. Here's how each component works:

#### 1. **Identity Management**: Creates cryptographic identity for server-side calls
```javascript
// Completely insecure seed phrase. Do not use for any purpose other than testing.
const seed = process.env.SEED_PHRASE || 'test test test test test test test test test test test test';

// Create identity from seed phrase
let identity;
try {
  identity = Ed25519KeyIdentity.fromSeedPhrase(seed);
} catch (error) {
  console.warn('⚠️  Could not create identity from seed phrase, generating random identity');
  identity = Ed25519KeyIdentity.generate();
}

// Initialize the agent with the identity
const agent = new HttpAgent({
  identity,
  host: process.env.DFX_NETWORK === 'local' ? 'http://127.0.0.1:4943' : 'https://ic0.app'
});
```

#### 2. **Request Encoding**: Converts JavaScript calls to Candid format
```javascript
// Create actor instance with the canister interface
const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId
});

// The agent automatically handles Candid encoding when you call canister methods
const result = await actor.get_likes(postId);
const posts = await actor.get_posts_with_likes();
```

#### 3. **Network Communication**: Handles HTTP requests to ICP network
```javascript
// Fetch root key for local development
if (process.env.DFX_NETWORK !== 'ic') {
  agent.fetchRootKey().catch(console.warn);
}

// The agent handles all network communication, retries, and routing
const newLikes = await actor.like(postId);
```

#### 4. **Response Processing**: Decodes and verifies canister responses
```javascript
// Handle the Result type from Candid
if (result.Ok !== undefined) {
  res.json({ 
    postId, 
    likes: result.Ok.toString(),
    message: 'Likes retrieved from ICP canister'
  });
} else if (result.Err !== undefined) {
  res.status(400).json({ 
    error: 'Failed to get likes',
    details: result.Err 
  });
} else {
  res.status(500).json({ 
    error: 'Unexpected response format from ICP canister'
  });
}
```

## 🔧 Configuration

### Environment Variables
Create a `.env` file in the `web2-api` directory:
```env
PORT=3000
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=canister-id
SEED_PHRASE=your-actual-secret-seed-phrase-here-make-it-long-enough
```

### Canister ID
The canister ID is automatically detected from your local deployment, but you can override it with the environment variable.

## ⚠️ Important Note

**Initialization Required**: After deploying the canister, you must initialize the sample posts by running:
```bash
dfx canister call post_likes_backend initialize_sample_posts
```

This is because the `init()` function only runs on fresh installations, not on upgrades. The initialization function creates the sample posts that your API will serve.

## 🚨 Error Handling

The system includes comprehensive error handling:

- **Invalid Post IDs**: Empty or malformed post identifiers
- **Network Issues**: ICP network connectivity problems
- **Canister Errors**: Runtime errors in the Rust canister
- **API Validation**: Input validation and sanitization

## 🔒 Security Considerations

- **Identity Management**: Server-side identity for canister calls
- **Input Validation**: Sanitize all user inputs
- **Error Messages**: Don't expose internal system details
- **Rate Limiting**: Consider implementing rate limiting for production

## 🚀 Next Steps

After completing this tutorial, you can:

1. **Add Authentication**: Implement user authentication for like operations
2. **Extend Data Model**: Add more fields to posts and likes
3. **Implement Caching**: Cache frequently accessed like counts
4. **Add Monitoring**: Track API performance and canister calls
5. **Deploy to Mainnet**: Move from local development to production

## 📚 Additional Resources

- [ICP JavaScript Agent Documentation](https://internetcomputer.org/docs/building-apps/interact-with-canisters/agents/javascript-agent)
- [Candid Interface Language](https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-intro)
- [DFINITY SDK Documentation](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- [ICP Developer Forum](https://forum.dfinity.org/)

## 🤝 Contributing

This tutorial is part of the ICP learning examples. Feel free to:
- Report issues
- Suggest improvements
- Add new features
- Share your own integration examples

## 📄 License

MIT License - feel free to use this code in your own projects!

---

**Happy Building!**

*This tutorial demonstrates the power of combining Web2 infrastructure with ICP's decentralized capabilities. Start simple, learn the patterns, and build something amazing!*
