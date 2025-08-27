# Post Likes Node.js Backend - Code Implementation Guide

This guide explains how to build a Node.js Web2 API that integrates with an ICP canister using the official JavaScript agent. We'll walk through the code structure and show you exactly how to call your backend canister from a Node.js service.

## 🏗️ Architecture Overview

The Node.js backend consists of a single `server.js` file that handles:
- Express.js web server setup
- ICP JavaScript agent integration
- HTTP endpoint handlers
- Canister communication

## 📁 Code Structure

### **Server Setup and Dependencies**

```javascript
import express from 'express';
import cors from 'cors';
import { HttpAgent } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { Actor } from '@dfinity/agent';
import { idlFactory } from '../src/declarations/post_likes_backend/post_likes_backend.did.js';
import dotenv from 'dotenv';
```
*Source: [server.js lines 1-10](server.js#L1-L10)*

Key imports:
- **`@dfinity/agent`**: Official ICP JavaScript agent for canister communication
- **`@dfinity/identity`**: Identity management for authentication
- **`idlFactory`**: Generated Candid interface from your canister

### **Environment Configuration**

```javascript
dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;
```
*Source: [server.js lines 12-13](server.js#L12-L13)*

The server loads environment variables and sets up the Express app with configurable port.

### **Identity Management**

```javascript
// Completely insecure seed phrase. Do not use for any purpose other than testing.
// Resolves to "rwbxt-jvr66-qvpbz-2kbh3-u226q-w6djk-b45cp-66ewo-tpvng-thbkh-wae"
const seed = process.env.SEED_PHRASE || 'test test test test test test test test test test test test';
if (!process.env.SEED_PHRASE) {
  console.warn('⚠️  WARNING: Using fallback seed phrase. Set SEED_PHRASE in .env for production!');
}

// Create identity from seed phrase - using the correct method for current version
let identity;
try {
  // For current versions, use fromSeedPhrase
  identity = Ed25519KeyIdentity.fromSeedPhrase(seed);
} catch (error) {
  console.warn('⚠️  Could not create identity from seed phrase, generating random identity');
  identity = Ed25519KeyIdentity.generate();
}
```
*Source: [server.js lines 15-35](server.js#L15-L35)*

**Identity Creation Process:**
1. **Seed Phrase**: Uses environment variable or fallback for testing
2. **Ed25519KeyIdentity**: Creates a deterministic identity from the seed phrase
3. **Fallback**: Generates random identity if seed phrase fails
4. **Security Warning**: Always use proper seed phrases in production

### **ICP Agent Initialization**

```javascript
// Initialize the agent
const agent = new HttpAgent({
  identity,
  host: process.env.DFX_NETWORK === 'local' ? 'http://127.0.0.1:4943' : 'https://ic0.app'
});

// Fetch root key for local development
if (process.env.DFX_NETWORK !== 'ic') {
  agent.fetchRootKey().catch(console.warn);
}
```
*Source: [server.js lines 37-47](server.js#L37-L47)*

**Agent Configuration:**
- **`identity`**: The Ed25519 identity for authentication
- **`host`**: Automatically switches between local development and mainnet
- **`fetchRootKey()`**: Required for local development to establish trust

### **Canister Actor Creation**

```javascript
// Create actor instance
const canisterId = process.env.POST_LIKES_BACKEND_CANISTER_ID || 'bkyz2-fmaaa-aaaaa-qaaaq-cai'; // Default local canister ID
const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId
});
```
*Source: [server.js lines 49-53](server.js#L49-L53)*

**Actor Setup:**
- **`idlFactory`**: Generated interface from your canister's Candid file
- **`agent`**: The configured HttpAgent instance
- **`canisterId`**: Target canister ID from environment or default

### **Route Handlers**

#### **Health Check Endpoint**

```javascript
// Routes
app.get('/health', (req, res) => {
  res.json({ status: 'OK', message: 'Web2 API is running and connected to ICP' });
});
```
*Source: [server.js lines 55-58](server.js#L55-L58)*

Simple health check to verify the service is running and connected to ICP.

#### **Get Likes Handler**

```javascript
// Get likes for a specific post
app.get('/likes/:postId', async (req, res) => {
  try {
    const { postId } = req.params;
    
    if (!postId || postId.trim() === '') {
      return res.status(400).json({ error: 'Post ID is required' });
    }

    console.log(`Fetching likes for post: ${postId}`);
    const result = await actor.get_likes(postId);
    
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
  } catch (error) {
    console.error('Error getting likes:', error);
    res.status(500).json({ 
      error: 'Failed to get likes from ICP canister',
      details: error.message 
    });
  }
});
```
*Source: [server.js lines 60-85](server.js#L60-L85)*

**Key Implementation Details:**
1. **Input Validation**: Checks if `postId` is provided and not empty
2. **Canister Call**: `actor.get_likes(postId)` calls the ICP canister method
3. **Result Handling**: Candid `Result` types have `Ok` or `Err` properties
4. **Error Handling**: Comprehensive error handling for both canister and network errors

#### **Create Post Handler**

```javascript
// Create a new post
app.post('/posts', async (req, res) => {
  try {
    const { id, title, content } = req.body;
    
    if (!id || !title || !content) {
      return res.status(400).json({ error: 'Post ID, title, and content are required' });
    }

    console.log(`Creating post: ${id}`);
    const result = await actor.create_post(id, title, content);
    
    // Handle the Result type from Candid
    if (result.Ok !== undefined) {
      res.json({ 
        post: result.Ok,
        message: 'Post created successfully on ICP canister'
      });
    } else if (result.Err !== undefined) {
      res.status(400).json({ 
        error: 'Failed to create post',
        details: result.Err 
      });
    } else {
      res.status(500).json({ 
        error: 'Unexpected response format from ICP canister'
      });
    }
  } catch (error) {
    console.error('Error creating post:', error);
    res.status(500).json({ 
      error: 'Failed to create post on ICP canister',
      details: error.message 
    });
  }
});
```
*Source: [server.js lines 87-120](server.js#L87-L120)*

**Create Post Process:**
1. **Request Validation**: Ensures all required fields are present
2. **Canister Update**: `actor.create_post()` modifies canister state
3. **Response Handling**: Returns the created post or error details

#### **Like Post Handler**

```javascript
// Like a specific post
app.post('/like/:postId', async (req, res) => {
  try {
    const { postId } = req.params;
    
    if (!postId || postId.trim() === '') {
      return res.status(400).json({ error: 'Post ID is required' });
    }

    console.log(`Liking post: ${postId}`);
    const result = await actor.like(postId);
    
    // Handle the Result type from Candid
    if (result.Ok !== undefined) {
      res.json({ 
        postId, 
        newLikes: result.Ok.toString(),
        message: 'Post liked successfully on ICP canister'
      });
    } else if (result.Err !== undefined) {
      res.status(400).json({ 
        error: 'Failed to like post',
        details: result.Err 
      });
    } else {
      res.status(500).json({ 
        error: 'Unexpected response format from ICP canister'
      });
    }
  } catch (error) {
    console.error('Error liking post:', error);
    res.status(500).json({ 
      error: 'Failed to like post on ICP canister',
      details: error.message 
    });
  }
});
```
*Source: [server.js lines 122-155](server.js#L122-L155)*

**Like Post Process:**
1. **Parameter Extraction**: Gets `postId` from URL parameters
2. **State Update**: `actor.like()` increments the like count
3. **Response Format**: Returns the new like count and success message

#### **Get Posts Handler**

```javascript
// Get all posts with their like counts
app.get('/posts', async (req, res) => {
  try {
    // Get posts directly from the ICP canister
    const postsWithLikes = await actor.get_posts_with_likes();
    
    // Transform the response to match our expected format
    const formattedPosts = postsWithLikes.map(([post, likes]) => ({
      id: post.id,
      title: post.title,
      content: post.content,
      created_at: post.created_at,
      likes: likes.toString()
    }));

    res.json({ 
      posts: formattedPosts,
      message: 'Posts retrieved directly from ICP canister'
    });
  } catch (error) {
    console.error('Error getting posts:', error);
    res.status(500).json({ 
      error: 'Failed to get posts from ICP canister',
      details: error.message 
    });
  }
});
```
*Source: [server.js lines 157-180](server.js#L157-L180)*

**Posts Retrieval Process:**
1. **Canister Query**: `actor.get_posts_with_likes()` fetches all posts
2. **Data Transformation**: Maps the raw canister response to a clean format
3. **Response Formatting**: Converts `Nat` types to strings for JSON compatibility

### **Server Startup**

```javascript
// Start server
app.listen(PORT, () => {
  console.log(`🚀 Web2 API server running on port ${PORT}`);
  console.log(`🔗 Health check: http://localhost:${PORT}/health`);
  console.log(`📊 Get likes: http://localhost:${PORT}/likes/{postId}`);
  console.log(`👍 Like post: POST http://localhost:${PORT}/like/{postId}`);
  console.log(`📝 Create post: POST http://localhost:${PORT}/posts`);
  console.log(`🌐 Connected to ICP canister: ${canisterId}`);
});
```
*Source: [server.js lines 182-208](server.js#L182-L208)*

**Server Information:**
- **Port Binding**: Listens on configured port
- **Endpoint Documentation**: Logs all available endpoints
- **Canister Connection**: Shows which canister is connected

## 🔑 Key Concepts for ICP Integration

### **Candid Result Types**
Your canister methods return `Result` types that must be handled:

```javascript
// Example of handling Candid Result type
const result = await actor.some_method();
if (result.Ok !== undefined) {
  // Success case
  return result.Ok;
} else if (result.Err !== undefined) {
  // Error case
  throw new Error(result.Err);
}
```

### **Update vs Query Methods**
- **Update methods** (like `create_post`, `like`): Modify canister state, require consensus
- **Query methods** (like `get_likes`, `get_posts`): Read-only, immediate response

### **Identity Management**
- **Seed Phrase**: Deterministic identity generation for consistent authentication
- **Ed25519**: Cryptographic algorithm used for identity verification
- **Local vs Mainnet**: Different identity handling for development vs production

### **Error Handling Patterns**
1. **Input Validation**: Check parameters before canister calls
2. **Canister Errors**: Handle `Result.Err` cases from canister responses
3. **Network Errors**: Catch exceptions from agent communication
4. **Response Formatting**: Ensure consistent error response structure

## 🚀 Running the Service

### Development Mode
```bash
npm start
```

### Production Mode
```bash
NODE_ENV=production npm start
```

## 🔧 Configuration

Create a `.env` file with:
```env
PORT=3000
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=your-canister-id
SEED_PHRASE=your-actual-secret-seed-phrase-here
```

## 📚 Next Steps

After understanding this implementation:
1. **Add authentication middleware** for secure endpoints
2. **Implement rate limiting** to prevent abuse
3. **Add request validation** with libraries like Joi or Zod
4. **Implement caching** for frequently accessed data
5. **Add monitoring and logging** for production deployment
6. **Set up proper identity management** for production use

This implementation demonstrates how to seamlessly integrate Node.js web services with ICP canisters, providing a bridge between traditional Web2 APIs and decentralized blockchain functionality.
