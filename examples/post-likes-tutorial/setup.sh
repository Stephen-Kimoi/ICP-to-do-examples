#!/bin/bash

echo "🚀 Setting up Tamper-Proof Post Likes Tutorial"
echo "================================================"

# Check if dfx is installed
if ! command -v dfx &> /dev/null; then
    echo "❌ DFINITY SDK (dfx) is not installed."
    echo "Please install it from: https://internetcomputer.org/docs/current/developer-docs/setup/install/"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed."
    echo "Please install Node.js 18+ from: https://nodejs.org/"
    exit 1
fi

echo "✅ Prerequisites check passed"
echo ""

# Install Web2 API dependencies
echo "📦 Installing Web2 API dependencies..."
cd web2-api
npm install
npm install node-fetch

if [ $? -eq 0 ]; then
    echo "✅ Web2 API dependencies installed successfully"
else
    echo "❌ Failed to install Web2 API dependencies"
    exit 1
fi

cd ..

echo ""
echo "🎯 Setup Complete!"
echo "=================="
echo ""
echo "Next steps:"
echo "1. Start local ICP network: dfx start --background"
echo "2. Deploy canister: dfx deploy post-likes-backend"
echo "3. Generate bindings: dfx generate post-likes-backend"
echo "4. Start Web2 API: cd web2-api && npm start"
echo "5. Test integration: node test-client.js"
echo ""
echo "📚 Check README.md for detailed instructions"
echo "🔗 Happy building! 🚀"
