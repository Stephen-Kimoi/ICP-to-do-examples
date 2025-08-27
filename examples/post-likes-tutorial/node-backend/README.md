# Node.js Backend for Post Likes Tutorial

This is the Node.js implementation of the post likes backend using the [ICP JavaScript Agent](https://internetcomputer.org/docs/building-apps/interact-with-canisters/agents/javascript-agent).

## Features

- **HTTP API Server**: Express.js server with RESTful endpoints
- **ICP Integration**: Uses DFINITY JavaScript agent for canister communication
- **Identity Management**: Server-side identity for secure canister calls
- **Error Handling**: Comprehensive error handling and validation

## API Endpoints

- `GET /health` - Health check
- `GET /likes/:postId` - Get likes for a post
- `POST /like/:postId` - Like a post
- `GET /posts` - Get all posts with like counts
- `POST /posts` - Create a new post

## Quick Start

### Prerequisites

- Node.js 18+ installed
- DFX running locally with the post-likes-backend canister deployed
- Environment variables configured

### Installation

```bash
cd node-backend
npm install
```

### Configuration

Create a `.env` file in the `node-backend` directory:

```env
PORT=3000
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=your-canister-id
SEED_PHRASE=your-seed-phrase-here
```

### Running the Server

```bash
npm start
```

The server will start on port 3000 (or the port specified in your .env file).

## Architecture

This backend demonstrates:

1. **Agent Initialization**: Setting up the ICP JavaScript agent
2. **Identity Management**: Creating and managing server-side identities
3. **Canister Communication**: Making query and update calls to the ICP canister
4. **Error Handling**: Proper error handling for canister responses
5. **API Design**: RESTful API design with proper HTTP status codes

## Key Implementation Details

### Agent Setup

```javascript
const agent = new HttpAgent({
  identity,
  host: process.env.DFX_NETWORK === 'local' ? 'http://127.0.0.1:4943' : 'https://ic0.app'
});
```

### Canister Calls

```javascript
const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId
});

const result = await actor.get_likes(postId);
```

## Testing

Use the included `test-client.js` to test the API endpoints:

```bash
node test-client.js
```

## Security Notes

- **Seed Phrase**: The current implementation uses a hardcoded seed phrase for testing. In production, use proper secret management.
- **Identity**: Consider implementing proper identity management for production use.
- **Validation**: All inputs are validated before processing.

## Troubleshooting

- **Canister Connection**: Ensure the canister is deployed and running
- **Network Issues**: Check your DFX_NETWORK configuration
- **Identity Errors**: Verify your seed phrase is correct
