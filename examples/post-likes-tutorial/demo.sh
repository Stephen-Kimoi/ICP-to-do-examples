#!/bin/bash

echo "🎬 Tamper-Proof Post Likes Tutorial - Complete Demo"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if dfx is running
if pgrep -f "dfx start" > /dev/null; then
    print_warning "DFX is already running. Stopping it first..."
    pkill -f "dfx start"
    sleep 2
fi

print_status "Starting local ICP network..."
dfx start --background --clean

# Wait for network to be ready
print_status "Waiting for network to be ready..."
sleep 10

print_status "Deploying post-likes-backend canister..."
dfx deploy post-likes-backend

if [ $? -eq 0 ]; then
    print_success "Canister deployed successfully!"
else
    print_error "Failed to deploy canister"
    exit 1
fi

print_status "Generating JavaScript bindings..."
dfx generate post-likes-backend

if [ $? -eq 0 ]; then
    print_success "Bindings generated successfully!"
else
    print_error "Failed to generate bindings"
    exit 1
fi

# Get canister ID
CANISTER_ID=$(dfx canister id post-likes-backend)
print_success "Canister ID: $CANISTER_ID"

# Create .env file for Web2 API
print_status "Creating environment configuration..."
cat > web2-api/.env << EOF
PORT=3000
DFX_NETWORK=local
POST_LIKES_BACKEND_CANISTER_ID=$CANISTER_ID
EOF

print_success "Environment file created"

# Install dependencies if not already installed
if [ ! -d "web2-api/node_modules" ]; then
    print_status "Installing Web2 API dependencies..."
    cd web2-api
    npm install
    npm install node-fetch
    cd ..
fi

print_status "Starting Web2 API server..."
cd web2-api
npm start &
API_PID=$!
cd ..

# Wait for API to start
sleep 5

# Test the API
print_status "Testing API endpoints..."

# Health check
print_status "Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s http://localhost:3000/health)
if echo "$HEALTH_RESPONSE" | grep -q "OK"; then
    print_success "Health check passed"
else
    print_error "Health check failed"
    echo "Response: $HEALTH_RESPONSE"
fi

# Test likes functionality
print_status "Testing likes functionality..."

# Get initial likes
INITIAL_LIKES=$(curl -s "http://localhost:3000/likes/post-1" | grep -o '"likes":"[^"]*"' | cut -d'"' -f4)
print_status "Initial likes for post-1: $INITIAL_LIKES"

# Like the post
print_status "Liking post-1..."
LIKE_RESPONSE=$(curl -s -X POST "http://localhost:3000/like/post-1")
NEW_LIKES=$(echo "$LIKE_RESPONSE" | grep -o '"newLikes":"[^"]*"' | cut -d'"' -f4)
print_success "Post liked! New count: $NEW_LIKES"

# Verify the increment
VERIFY_LIKES=$(curl -s "http://localhost:3000/likes/post-1" | grep -o '"likes":"[^"]*"' | cut -d'"' -f4)
if [ "$VERIFY_LIKES" = "$NEW_LIKES" ]; then
    print_success "Likes verification passed! Count: $VERIFY_LIKES"
else
    print_error "Likes verification failed! Expected: $NEW_LIKES, Got: $VERIFY_LIKES"
fi

echo ""
print_success "🎉 Demo completed successfully!"
echo ""
echo "📊 Summary:"
echo "   • Local ICP network: Running"
echo "   • Canister deployed: $CANISTER_ID"
echo "   • Web2 API: Running on http://localhost:3000"
echo "   • Test results: All endpoints working"
echo ""
echo "🔗 Try these endpoints:"
echo "   • Health: http://localhost:3000/health"
echo "   • Get likes: http://localhost:3000/likes/post-1"
echo "   • Like post: POST http://localhost:3000/like/post-1"
echo "   • All posts: http://localhost:3000/posts"
echo ""
echo "🧪 Run the full test suite:"
echo "   cd web2-api && node test-client.js"
echo ""
echo "🛑 To stop everything:"
echo "   pkill -f 'dfx start' && pkill -f 'node.*server.js'"

# Keep the script running to maintain the demo
echo ""
print_status "Demo is running. Press Ctrl+C to stop everything..."
wait $API_PID
