#!/bin/bash

# AI Core API Test Script
# Pastikan server sudah berjalan: cargo run

# Configuration
BASE_URL="http://127.0.0.1:3000"
TOKEN="rahasia_token_anda_yang_kuat_123456"

echo "🧪 Testing AI Core API"
echo "====================="
echo ""

# Test 1: Health Check (Public)
echo "📍 Test 1: Health Check"
curl -s "$BASE_URL/health" | jq '.'
echo ""
echo ""

# Test 2: Create Experience
echo "📍 Test 2: Create Experience"
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "User bertanya tentang cuaca hari ini",
    "source": "user",
    "metadata": "weather_query"
  }' \
  "$BASE_URL/experiences" | jq '.'
echo ""
echo ""

# Test 3: Create More Experiences
echo "📍 Test 3: Create Multiple Experiences"
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Cuaca hari ini cerah", "source": "system"}' \
  "$BASE_URL/experiences" | jq '.'

curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "User senang dengan cuaca", "source": "user"}' \
  "$BASE_URL/experiences" | jq '.'

curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Cuaca besok akan hujan", "source": "system"}' \
  "$BASE_URL/experiences" | jq '.'
echo ""
echo ""

# Test 4: Get All Experiences
echo "📍 Test 4: Get All Experiences"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/experiences" | jq '.data | length' | xargs -I {} echo "Total experiences: {}"
echo ""
echo ""

# Test 5: Search Experiences
echo "📍 Test 5: Search Experiences (keyword: cuaca)"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/experiences/search?q=cuaca" | jq '.data | length' | xargs -I {} echo "Found {} experiences"
echo ""
echo ""

# Test 6: Get Statistics
echo "📍 Test 6: Get Statistics"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/stats" | jq '.data'
echo ""
echo ""

# Test 7: Get Pattern Detail
echo "📍 Test 7: Get Pattern Detail (keyword: cuaca)"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/patterns/cuaca" | jq '.data'
echo ""
echo ""

# Test 8: Interact with AI
echo "📍 Test 8: Interact with AI"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/interact" | jq '.data'
echo ""
echo ""

# Test 9: Make Decision
echo "📍 Test 9: Make Decision"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/decision" | jq '.data'
echo ""
echo ""

# Test 10: Make Decision for Query
echo "📍 Test 10: Make Decision for Query (q: cuaca)"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/decision/query?q=cuaca" | jq '.data'
echo ""
echo ""

# Test 11: Update Personality
echo "📍 Test 11: Update Personality"
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "input": "User bertanya dengan sopan dan penuh perhatian",
    "response": "Tentu, saya akan membantu Anda"
  }' \
  "$BASE_URL/personality" | jq '.data'
echo ""
echo ""

# Test 12: Reflect Memory
echo "📍 Test 12: Reflect Memory"
curl -s -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/reflect" | jq '.data.total_experiences' | xargs -I {} echo "Total experiences in reflection: {}"
echo ""
echo ""

# Test 13: Test Unauthorized Access
echo "📍 Test 13: Test Unauthorized Access (should fail)"
curl -s -H "Authorization: Bearer wrong_token" \
  "$BASE_URL/experiences" 
echo ""
echo ""

# Test 14: Clear Patterns
echo "📍 Test 14: Clear and Rebuild Patterns"
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/patterns/clear" | jq '.'
echo ""
echo ""

echo "✅ All tests completed!"
echo ""
echo "💡 Tips:"
echo "  - Check server logs for detailed output"
echo "  - Memory auto-saves every 60 seconds to data/memory.json"
echo "  - Use /memory/clear to reset all data"
