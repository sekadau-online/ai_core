# 🚀 Quick Start Guide - AI Core

Panduan cepat untuk menjalankan AI Core dalam 5 menit!

## 1️⃣ Setup Environment (1 menit)

```bash
# Buat file .env
cat > .env << 'EOF'
BEARER_TOKEN=rahasia_token_anda_123456
HOST=127.0.0.1
PORT=3000
EOF
```

**Windows PowerShell:**
```powershell
@"
BEARER_TOKEN=rahasia_token_anda_123456
HOST=127.0.0.1
PORT=3000
"@ | Out-File -FilePath .env -Encoding UTF8
```

## 2️⃣ Build & Run (2 menit)

**Linux/WSL:**
```bash
cargo run --release
```

**Output yang diharapkan:**
```
🚀 Starting AI Core API
   Bearer Token configured: true
   Starting with fresh memory
   API listening on http://127.0.0.1:3000
```

## 3️⃣ Test API (2 menit)

### Test 1: Health Check (Public - No Auth)
```bash
curl http://localhost:3000/health
```

Expected:
```json
{"success":true,"data":"AI Core is running","message":"OK"}
```

### Test 2: Create Experience (Protected - With Auth)
```bash
TOKEN="rahasia_token_anda_123456"

curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello AI","source":"user"}' \
  http://localhost:3000/experiences
```

Expected:
```json
{
  "success": true,
  "data": {
    "id": "exp_...",
    "content": "Hello AI",
    "source": "user",
    "timestamp": "2025-11-10T..."
  },
  "message": "Experience created successfully"
}
```

### Test 3: Get Statistics
```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/stats
```

Expected:
```json
{
  "success": true,
  "data": {
    "total_experiences": 1,
    "total_patterns": 2,
    "top_patterns": [...]
  }
}
```

## ✅ Selesai!

Server sudah berjalan dan siap digunakan!

## 📚 Next Steps

1. **Baca dokumentasi lengkap**: `API_DOCUMENTATION.md`
2. **Import Postman collection**: `postman_collection.json`
3. **Jalankan test suite**: `./test_api.sh`
4. **Explore 15+ endpoints** yang tersedia

## 🔧 Common Issues

### Issue: "cargo: command not found"
**Solution**: Pastikan Rust terinstall
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Issue: "Port 3000 already in use"
**Solution**: Ubah port di `.env`
```bash
PORT=8080
```

### Issue: "401 Unauthorized"
**Solution**: Pastikan token benar
```bash
# Check token di .env
cat .env | grep BEARER_TOKEN

# Gunakan token yang sama di curl
curl -H "Authorization: Bearer TOKEN_DARI_ENV" ...
```

## 💡 Pro Tips

1. **Auto-reload memory**: Memory disimpan otomatis setiap 60 detik
2. **Check logs**: Server menampilkan log untuk setiap request
3. **Use Postman**: Import collection untuk testing yang lebih mudah
4. **Production ready**: Sudah siap untuk production dengan HTTPS proxy

## 🎯 Key Endpoints Cheat Sheet

```bash
TOKEN="your_token_here"

# Create experience
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content":"text","source":"user"}' \
  http://localhost:3000/experiences

# Get all
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/experiences

# Search
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:3000/experiences/search?q=hello"

# Stats
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/stats

# Decision
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/decision

# Personality
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"input":"hello","response":"hi"}' \
  http://localhost:3000/personality

# Clear memory
curl -X DELETE -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/memory/clear
```

## 🎉 Happy Coding!

Untuk bantuan lebih lanjut, lihat:
- `README.md` - Complete documentation
- `API_DOCUMENTATION.md` - API reference
- `SUMMARY.md` - Implementation details
