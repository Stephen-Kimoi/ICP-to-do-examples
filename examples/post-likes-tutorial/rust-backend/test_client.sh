#!/bin/bash

# Test client for the Rust backend API
BASE_URL="http://localhost:3001"

echo "🚀 Starting Rust Backend API Tests"
echo "=================================================="

# Test health check
echo "🔍 Testing health check..."
response=$(curl -s -w "%{http_code}" "$BASE_URL/health")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ Health check passed: $(echo $body | jq -r '.message')"
else
    echo "❌ Health check failed: $http_code"
fi

# Test getting posts
echo -e "\n📝 Testing get posts..."
response=$(curl -s -w "%{http_code}" "$BASE_URL/posts")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ Got posts: $(echo $body | jq -r '.message')"
    echo "$body" | jq -r '.posts[] | "   - \(.id): \(.title) (\(.likes) likes)"'
else
    echo "❌ Get posts failed: $http_code"
fi

# Test getting likes for a specific post
echo -e "\n👍 Testing get likes for post-1..."
response=$(curl -s -w "%{http_code}" "$BASE_URL/likes/post-1")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ post-1 has $(echo $body | jq -r '.likes') likes: $(echo $body | jq -r '.message')"
else
    echo "❌ Get likes failed: $http_code"
fi

# Test liking a post
echo -e "\n❤️ Testing like post post-1..."
response=$(curl -s -w "%{http_code}" -X POST "$BASE_URL/like/post-1")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ post-1 now has $(echo $body | jq -r '.new_likes') likes: $(echo $body | jq -r '.message')"
else
    echo "❌ Like post failed: $http_code"
fi

# Test getting likes again to see the change
echo -e "\n👍 Testing get likes for post-1 again..."
response=$(curl -s -w "%{http_code}" "$BASE_URL/likes/post-1")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ post-1 has $(echo $body | jq -r '.likes') likes: $(echo $body | jq -r '.message')"
else
    echo "❌ Get likes failed: $http_code"
fi

# Test creating a new post
echo -e "\n✏️ Testing create post post-test..."
post_data='{"id": "post-test", "title": "Test Post from Rust Client", "content": "This is a test post created by the Rust test client."}'
response=$(curl -s -w "%{http_code}" -X POST "$BASE_URL/posts" \
    -H "Content-Type: application/json" \
    -d "$post_data")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ Post created: $(echo $body | jq -r '.message')"
    if [ "$(echo $body | jq -r '.data')" != "null" ]; then
        echo "   - ID: $(echo $body | jq -r '.data.id')"
        echo "   - Title: $(echo $body | jq -r '.data.title')"
    fi
else
    echo "❌ Create post failed: $http_code"
fi

# Test getting posts again to see the new post
echo -e "\n📝 Testing get posts again..."
response=$(curl -s -w "%{http_code}" "$BASE_URL/posts")
http_code="${response: -3}"
body="${response%???}"

if [ "$http_code" -eq 200 ]; then
    echo "✅ Got posts: $(echo $body | jq -r '.message')"
    echo "$body" | jq -r '.posts[] | "   - \(.id): \(.title) (\(.likes) likes)"'
else
    echo "❌ Get posts failed: $http_code"
fi

echo -e "\n=================================================="
echo "🎉 All tests completed!"
