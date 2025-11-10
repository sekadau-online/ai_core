# 🧪 Testing Guide - Server is Running!

## ✅ Server Status
```
✅ Server is running on http://127.0.0.1:3000
✅ Bearer Token: rahasia_token_anda_yang_kuat_123456
✅ Memory loaded from file
✅ Auto-save every 60 seconds
```

## 🔑 Authentication
All protected endpoints require Bearer token:
```bash
TOKEN="rahasia_token_anda_yang_kuat_123456"
```

---

## 📝 Quick Test Commands

### 1. Health Check (Public - No Auth)
```bash
curl http://127.0.0.1:3000/health
```

Expected response:
```json
{
  "success": true,
  "data": "AI Core is running",
  "message": "OK"
}
```

---

### 2. Create Experience
```bash
TOKEN="rahasia_token_anda_yang_kuat_123456"

curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello AI Core","source":"user"}' \
  http://127.0.0.1:3000/experiences
```

---

### 3. Get All Experiences
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/experiences
```

---

### 4. Search Experiences
```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:3000/experiences/search?q=hello"
```

---

### 5. Get Statistics
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/stats
```

Response shows:
- Total experiences
- Total patterns found
- Top 10 patterns with frequencies

---

### 6. Interact with AI (Watch Server Logs!)
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/interact
```

**Server will show:**
```
💬 Interaction Summary:
   Total experiences: X
   Analyzing patterns...
[Pattern details]
```

---

### 7. Make Decision
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/decision
```

---

### 8. Make Decision for Query
```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://127.0.0.1:3000/decision/query?q=what+should+I+do"
```

---

### 9. Update Personality
```bash
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "input": "halo! terima kasih banyak atas bantuannya!",
    "response": "Sama-sama, senang membantu"
  }' \
  http://127.0.0.1:3000/personality
```

Expected response:
```json
{
  "success": true,
  "data": {
    "curiosity": 0.5,
    "happiness": 0.7,  // Increased!
    "caution": 0.5,
    "dominant_trait": "happy",
    "influenced_response": "😊 Sama-sama, senang membantu"
  }
}
```

---

### 10. Reflect Memory (Watch Server Logs!)
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/reflect
```

**Server will show:**
```
📜 Reflection (X experiences):
- [2025-11-10 13:38:00] user → Experience 1
- [2025-11-10 13:39:00] system → Experience 2
...
```

---

### 11. Get Pattern Detail
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/patterns/hello
```

Shows:
- Keyword frequency
- Related experience IDs
- Related experience contents

---

### 12. Clear Patterns Cache
```bash
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/patterns/clear
```

---

### 13. Clear Memory (⚠️ Deletes All!)
```bash
curl -X DELETE \
  -H "Authorization: Bearer $TOKEN" \
  http://127.0.0.1:3000/memory/clear
```

---

## 🎯 Using the Test Script

Make it executable and run:
```bash
chmod +x test_quick.sh
./test_quick.sh
```

This will test all major endpoints automatically!

---

## 📊 Monitoring Server Logs

Server logs show:
- ✅ Successful requests
- ⚠️ Invalid token attempts (you saw these)
- 📝 Interaction summaries
- 📜 Memory reflections
- 💾 Auto-save confirmations

---

## 🐛 Common Issues

### Issue: "Invalid token attempt" warning
**Cause**: Wrong token in Authorization header

**Solution**: Use the correct token from `.env`:
```bash
TOKEN="rahasia_token_anda_yang_kuat_123456"
curl -H "Authorization: Bearer $TOKEN" ...
```

### Issue: 401 Unauthorized
**Cause**: Missing or incorrect Bearer token

**Solution**: Always include the header:
```bash
-H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456"
```

### Issue: Connection refused
**Cause**: Server not running

**Solution**: Start server in another terminal:
```bash
cargo run --release
```

---

## 💡 Pro Tips

1. **Watch Logs**: Keep server terminal visible to see:
   - Pattern analysis output
   - Memory reflections
   - Auto-save confirmations

2. **Test with jq**: Format JSON responses nicely:
   ```bash
   curl ... | jq '.'
   ```

3. **Check Persistence**: View saved data:
   ```bash
   cat data/memory.json | jq '.'
   ```

4. **Multiple Terminals**: 
   - Terminal 1: Server (`cargo run`)
   - Terminal 2: Testing commands

5. **Postman Alternative**: Import `postman_collection.json` for GUI testing

---

## 🎉 Your Server is Working Perfectly!

The warnings you saw were just invalid token attempts. The server is:
- ✅ Running correctly
- ✅ Authenticating properly
- ✅ Rejecting invalid tokens (security working!)
- ✅ Auto-saving memory
- ✅ Loading existing memory on startup

**Everything is working as expected!** 🚀

---

## 📚 Next Steps

1. Run `./test_quick.sh` for automated testing
2. Try the curl commands above manually
3. Import `postman_collection.json` to Postman
4. Check `API_DOCUMENTATION.md` for complete reference
5. Build your application on top of this API!
