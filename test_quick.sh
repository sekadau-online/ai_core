#!/bin/bash

# AI Core - Quick Test Commands
# Server is running! Use these commands to test the API

# ============================================
# CONFIGURATION
# ============================================
TOKEN="rahasia_token_anda_yang_kuat_123456"
BASE_URL="http://127.0.0.1:3000"

echo "🚀 AI Core API - Quick Test Commands"
echo "===================================="
echo ""

# ============================================
# PUBLIC ENDPOINTS (No Auth Required)
# ============================================
echo "📍 1. Health Check (Public)"
echo "Command:"
echo "curl $BASE_URL/health"
echo ""
echo "Running..."
curl -s $BASE_URL/health | jq '.'
echo ""
echo ""

# ============================================
# PROTECTED ENDPOINTS (Auth Required)
# ============================================

echo "📍 2. Create Experience"
echo "Command:"
echo "curl -X POST -H 'Authorization: Bearer $TOKEN' -H 'Content-Type: application/json' -d '{\"content\":\"Testing AI Core\",\"source\":\"user\"}' $BASE_URL/experiences"
echo ""
echo "Running..."
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content":"Testing AI Core API","source":"user"}' \
  $BASE_URL/experiences | jq '.'
echo ""
echo ""

echo "📍 3. Get All Experiences"
echo "Command:"
echo "curl -H 'Authorization: Bearer $TOKEN' $BASE_URL/experiences"
echo ""
echo "Running..."
curl -s -H "Authorization: Bearer $TOKEN" \
  $BASE_URL/experiences | jq '.data | length' | xargs -I {} echo "Total experiences: {}"
echo ""
echo ""

echo "📍 4. Get Statistics"
echo "Command:"
echo "curl -H 'Authorization: Bearer $TOKEN' $BASE_URL/stats"
echo ""
echo "Running..."
curl -s -H "Authorization: Bearer $TOKEN" \
  $BASE_URL/stats | jq '.data'
echo ""
echo ""

echo "📍 5. Interact with AI (with logging)"
echo "Command:"
echo "curl -H 'Authorization: Bearer $TOKEN' $BASE_URL/interact"
echo ""
echo "Running..."
curl -s -H "Authorization: Bearer $TOKEN" \
  $BASE_URL/interact | jq '.data'
echo ""
echo "Check server logs for detailed interaction output!"
echo ""
echo ""

echo "📍 6. Make Decision"
echo "Command:"
echo "curl -H 'Authorization: Bearer $TOKEN' $BASE_URL/decision"
echo ""
echo "Running..."
curl -s -H "Authorization: Bearer $TOKEN" \
  $BASE_URL/decision | jq '.data'
echo ""
echo ""

echo "📍 7. Update Personality"
echo "Command:"
echo "curl -X POST -H 'Authorization: Bearer $TOKEN' -H 'Content-Type: application/json' -d '{\"input\":\"terima kasih banyak!\",\"response\":\"Sama-sama\"}' $BASE_URL/personality"
echo ""
echo "Running..."
curl -s -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"input":"halo, terima kasih banyak!","response":"Sama-sama, senang membantu"}' \
  $BASE_URL/personality | jq '.data'
echo ""
echo ""

echo "📍 8. Reflect Memory (with logging)"
echo "Command:"
echo "curl -H 'Authorization: Bearer $TOKEN' $BASE_URL/reflect"
echo ""
echo "Running..."
curl -s -H "Authorization: Bearer $TOKEN" \
  $BASE_URL/reflect | jq '.data.total_experiences' | xargs -I {} echo "Total experiences in reflection: {}"
echo ""
echo "Check server logs for detailed reflection output!"
echo ""
echo ""

echo "✅ All tests completed!"
echo ""
echo "💡 Tips:"
echo "  - Server logs show detailed output for /interact and /reflect endpoints"
echo "  - Memory auto-saves every 60 seconds"
echo "  - Check data/memory.json for persistent storage"
echo ""
echo "🔗 More endpoints:"
echo "  - GET  /experiences/search?q=keyword"
echo "  - GET  /experiences/:id"
echo "  - GET  /patterns/:keyword"
echo "  - POST /patterns/clear"
echo "  - GET  /decision/query?q=question"
echo "  - DELETE /memory/clear"
