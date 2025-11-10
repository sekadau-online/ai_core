#!/bin/bash

echo "🚀 Installing AI Core Frontend Dependencies"
echo "=========================================="
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed!"
    echo "📥 Please install Node.js from: https://nodejs.org/"
    exit 1
fi

echo "✅ Node.js version: $(node --version)"
echo "✅ npm version: $(npm --version)"
echo ""

# Navigate to frontend directory
cd "$(dirname "$0")"

echo "📦 Installing dependencies..."
npm install

echo ""
echo "✅ Dependencies installed successfully!"
echo ""
echo "📝 Next steps:"
echo "   1. Ensure backend is running: cargo run"
echo "   2. Start frontend: npm start"
echo "   3. Open browser: http://localhost:8080"
echo ""
echo "🎉 Ready to go!"
