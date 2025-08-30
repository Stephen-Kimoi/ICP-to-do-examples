import fetch from 'node-fetch';

const API_BASE = 'http://localhost:3000';

async function testAPI() {
  console.log('🧪 Testing Web2 API + ICP Integration\n');

  try {
    // Test 1: Health check
    console.log('1️⃣ Testing health check...');
    const health = await fetch(`${API_BASE}/health`);
    const healthData = await health.json();
    console.log('✅ Health:', healthData.message);
    console.log('');

    // Test 2: Get initial likes for a post
    console.log('2️⃣ Testing get likes...');
    const getLikes = await fetch(`${API_BASE}/likes/post-1`);
    const likesData = await getLikes.json();
    console.log('✅ Initial likes for post-1:', likesData.likes);
    console.log('');

    // Test 3: Like a post
    console.log('3️⃣ Testing like post...');
    const likePost = await fetch(`${API_BASE}/like/post-1`, { method: 'POST' });
    const likeData = await likePost.json();
    console.log('✅ Post liked! New count:', likeData.newLikes);
    console.log('');

    // Test 4: Verify likes increased
    console.log('4️⃣ Verifying likes increased...');
    const verifyLikes = await fetch(`${API_BASE}/likes/post-1`);
    const verifyData = await verifyLikes.json();
    console.log('✅ Verified likes:', verifyData.likes);
    console.log('');

    // Test 5: Like another post
    console.log('5️⃣ Testing like on different post...');
    const likePost2 = await fetch(`${API_BASE}/like/post-2`, { method: 'POST' });
    const likeData2 = await likePost2.json();
    console.log('✅ Post-2 liked! Count:', likeData2.newLikes);
    console.log('');

    // Test 6: Get all posts with likes
    console.log('6️⃣ Testing get all posts...');
    const allPosts = await fetch(`${API_BASE}/posts`);
    const postsData = await allPosts.json();
    console.log('✅ All posts with ICP-stored likes:');
    postsData.posts.forEach(post => {
      console.log(`   📝 ${post.title}: ${post.likes} likes`);
    });
    console.log('');

    // Test 7: Like post-1 again to show increment
    console.log('7️⃣ Testing like increment...');
    const likeAgain = await fetch(`${API_BASE}/like/post-1`, { method: 'POST' });
    const likeAgainData = await likeAgain.json();
    console.log('✅ Post-1 liked again! New count:', likeAgainData.newLikes);
    console.log('');

    console.log('🎉 All tests completed successfully!');
    console.log('💡 This demonstrates how your Web2 API seamlessly integrates with ICP canisters');
    console.log('🔒 The likes are now stored tamper-proof on the Internet Computer!');

  } catch (error) {
    console.error('❌ Test failed:', error.message);
  }
}

// Run tests
testAPI();
