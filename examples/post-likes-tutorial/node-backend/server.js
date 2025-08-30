import express from 'express';
import cors from 'cors';
import { HttpAgent } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { Actor } from '@dfinity/agent';
import { idlFactory } from '../src/declarations/post_likes_backend/post_likes_backend.did.js';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json());

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

// Initialize the agent
const agent = new HttpAgent({
  identity,
  host: process.env.DFX_NETWORK === 'local' ? 'http://127.0.0.1:4943' : 'https://ic0.app'
});

// Fetch root key for local development
if (process.env.DFX_NETWORK !== 'ic') {
  agent.fetchRootKey().catch(console.warn);
}

// Create actor instance
const canisterId = process.env.POST_LIKES_BACKEND_CANISTER_ID || 'bkyz2-fmaaa-aaaaa-qaaaq-cai'; // Default local canister ID
const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId
});

// Routes
app.get('/health', (req, res) => {
  res.json({ status: 'OK', message: 'Web2 API is running and connected to ICP' });
});

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

// Start server
app.listen(PORT, () => {
  console.log(`Web2 API server running on port ${PORT}`);
  console.log(`🔗 Health check: http://localhost:${PORT}/health`);
  console.log(`📊 Get likes: http://localhost:${PORT}/likes/{postId}`);
  console.log(`👍 Like post: POST http://localhost:${PORT}/like/{postId}`);
  console.log(`📝 Create post: POST http://localhost:${PORT}/posts`);
  console.log(`🌐 Connected to ICP canister: ${canisterId}`);
});
