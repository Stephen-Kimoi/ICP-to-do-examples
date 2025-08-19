import express from 'express';
import cors from 'cors';
import { HttpAgent } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { Actor } from '@dfinity/agent';
import { idlFactory } from '../src/declarations/post-likes-backend/post-likes-backend.did.js';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json());

// Create a simple identity for server-side calls
const seed = 'your-secret-seed-phrase-here-make-it-long-enough';
const identity = Ed25519KeyIdentity.fromSeedPhrase(seed);

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
const canisterId = process.env.POST_LIKES_BACKEND_CANISTER_ID || 'rrkah-fqaaa-aaaaa-aaaaq-cai'; // Default local canister ID
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
    const likes = await actor.get_likes(postId);
    
    res.json({ 
      postId, 
      likes: likes.toString(),
      message: 'Likes retrieved from ICP canister'
    });
  } catch (error) {
    console.error('Error getting likes:', error);
    res.status(500).json({ 
      error: 'Failed to get likes from ICP canister',
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
    const newLikes = await actor.like(postId);
    
    res.json({ 
      postId, 
      newLikes: newLikes.toString(),
      message: 'Post liked successfully on ICP canister'
    });
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
    // For demo purposes, we'll return some sample posts
    // In a real app, you might store post metadata in your Web2 database
    const samplePosts = [
      { id: 'post-1', title: 'Getting Started with ICP', content: 'Learn the basics...' },
      { id: 'post-2', title: 'Web2 + ICP Integration', content: 'Bridge the gap...' },
      { id: 'post-3', title: 'Tamper-Proof Data', content: 'Why decentralization matters...' }
    ];

    // Get likes for each post from ICP
    const postsWithLikes = await Promise.all(
      samplePosts.map(async (post) => {
        try {
          const likes = await actor.get_likes(post.id);
          return { ...post, likes: likes.toString() };
        } catch (error) {
          console.warn(`Failed to get likes for ${post.id}:`, error.message);
          return { ...post, likes: '0' };
        }
      })
    );

    res.json({ 
      posts: postsWithLikes,
      message: 'Posts retrieved with ICP-stored like counts'
    });
  } catch (error) {
    console.error('Error getting posts:', error);
    res.status(500).json({ 
      error: 'Failed to get posts with likes',
      details: error.message 
    });
  }
});

// Start server
app.listen(PORT, () => {
  console.log(`🚀 Web2 API server running on port ${PORT}`);
  console.log(`🔗 Health check: http://localhost:${PORT}/health`);
  console.log(`📊 Get likes: http://localhost:${PORT}/likes/{postId}`);
  console.log(`👍 Like post: POST http://localhost:${PORT}/like/{postId}`);
  console.log(`📝 Get all posts: http://localhost:${PORT}/posts`);
  console.log(`🌐 Connected to ICP canister: ${canisterId}`);
});
