# 🎉 AI Core + Frontend - Complete Setup Guide

## 📋 Prerequisites

- Rust & Cargo installed
- Node.js 14+ installed
- WSL/Linux environment (for Rust backend)

## 🚀 Quick Setup (5 Minutes)

### Step 1: Setup Backend (AI Core API)

```bash
# Navigate to project root
cd /mnt/c/Users/f/ai_core

# Ensure .env is configured
cat .env
# Should show:
# BEARER_TOKEN=rahasia_token_anda_yang_kuat_123456
# HOST=127.0.0.1
# PORT=3000

# Start backend
cargo run --release
```

**Expected output:**
```
🚀 Starting AI Core API
   Bearer Token configured: true
   API listening on http://127.0.0.1:3000
```

### Step 2: Setup Frontend (Node.js)

Open a **new terminal**:

```bash
# Navigate to frontend directory
cd /mnt/c/Users/f/ai_core/frontend

# Install dependencies (first time only)
npm install

# Start frontend server
npm start
```

**Expected output:**
```
🌐 AI Core Frontend Server
==========================
✅ Frontend: http://localhost:8080
🔗 Backend:  http://127.0.0.1:3000
🔑 Token:    Configured

📝 Open your browser at http://localhost:8080
```

### Step 3: Open Browser

Navigate to: **http://localhost:8080**

You should see the AI Core Dashboard! 🎉

## 🎯 Usage Guide

### Create Your First Experience

1. Click "Create" in navigation
2. Enter content: "Hello AI Core!"
3. Select source: "user"
4. Click "Save Experience"
5. View in "Experiences" page

### View Statistics

1. Click "Statistics" in navigation
2. See total experiences and patterns
3. Click "View Details" on any pattern

### Update Personality

1. Click "Personality" in navigation
2. Enter input: "halo! terima kasih banyak!"
3. Enter response: "Sama-sama"
4. Click "Update Personality"
5. See traits update with progress bars

### Make Decision

1. Click "Decision" in navigation
2. Enter query (optional): "what should I do?"
3. Click "Make Decision"
4. See AI decision with confidence and reasoning

## 📂 Project Structure

```
ai_core/
├── Cargo.toml                 # Rust dependencies
├── .env                       # Backend config
├── src/                       # Rust source code
│   ├── main.rs               # Backend server
│   ├── api.rs                # API handlers
│   └── ...                   # Other modules
└── frontend/                  # Frontend application
    ├── package.json          # Node dependencies
    ├── .env                  # Frontend config
    ├── server.js             # Express server
    ├── views/                # EJS templates
    │   ├── index.ejs
    │   ├── experiences.ejs
    │   └── ...
    └── public/               # Static files
        ├── css/style.css
        └── js/main.js
```

## 🔧 Configuration Files

### Backend `.env` (root directory)
```env
BEARER_TOKEN=rahasia_token_anda_yang_kuat_123456
HOST=127.0.0.1
PORT=3000
```

### Frontend `.env` (frontend directory)
```env
BACKEND_URL=http://127.0.0.1:3000
BEARER_TOKEN=rahasia_token_anda_yang_kuat_123456
PORT=8080
```

**⚠️ IMPORTANT**: Both tokens must match!

## 🎨 Features Overview

### Backend (Rust API)
- ✅ 15+ REST API endpoints
- ✅ Bearer token authentication
- ✅ Memory persistence (JSON)
- ✅ Pattern recognition
- ✅ Decision making
- ✅ Personality system
- ✅ Auto-save every 60s

### Frontend (Node.js Web UI)
- ✅ Modern responsive design
- ✅ 9 interactive pages
- ✅ Real-time API status
- ✅ Toast notifications
- ✅ Modal dialogs
- ✅ Search functionality
- ✅ Pattern visualization
- ✅ Personality traits display

## 🧪 Testing

### Manual Testing

1. **Create Experience**:
   - Navigate to `/create`
   - Fill form and submit
   - Check `/experiences` page

2. **Search**:
   - Go to `/search`
   - Enter keyword: "hello"
   - View results

3. **Statistics**:
   - Go to `/stats`
   - See patterns
   - Click "View Details"

4. **Personality**:
   - Go to `/personality`
   - Try input: "apa itu AI?" (curiosity ↑)
   - Try input: "terima kasih!" (happiness ↑)
   - Try input: "error warning!" (caution ↑)

### API Testing (Backend Only)

```bash
TOKEN="rahasia_token_anda_yang_kuat_123456"

# Health check
curl http://localhost:3000/health

# Create experience
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content":"Test","source":"user"}' \
  http://localhost:3000/experiences

# Get stats
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/stats
```

## 🐛 Troubleshooting

### Backend Issues

**Problem**: "cargo: command not found"
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Problem**: "Port 3000 already in use"
```bash
# Change PORT in backend .env
PORT=3001
# Update BACKEND_URL in frontend .env
BACKEND_URL=http://127.0.0.1:3001
```

### Frontend Issues

**Problem**: "npm: command not found"
```bash
# Install Node.js
# Download from: https://nodejs.org/
```

**Problem**: "Cannot connect to backend"
```bash
# Ensure backend is running
# Terminal 1: cargo run
# Terminal 2: cd frontend && npm start
```

**Problem**: "401 Unauthorized"
```bash
# Check tokens match
cat .env | grep BEARER_TOKEN
cat frontend/.env | grep BEARER_TOKEN
```

### Browser Issues

**Problem**: "API Status: Offline"
- Backend not running → Start with `cargo run`
- Wrong URL → Check `BACKEND_URL` in frontend/.env

**Problem**: "Network Error"
- CORS issue → Backend handles CORS automatically
- Token mismatch → Check both .env files

## 📊 Monitoring

### Backend Logs
Watch backend terminal for:
- 🪶 Memory operations
- 💬 Interaction summaries
- 📜 Reflection outputs
- 💾 Auto-save confirmations

### Frontend Logs
Watch frontend terminal for:
- API calls
- Route access
- Errors

### Browser Console
Open DevTools (F12) to see:
- JavaScript logs
- API responses
- Network requests

## 🚀 Production Deployment

### Backend (Rust)
```bash
cargo build --release
./target/release/ai_core
```

### Frontend (Node.js)
```bash
cd frontend
npm install --production
NODE_ENV=production node server.js
```

### Using PM2
```bash
# Install PM2
npm install -g pm2

# Start backend (in WSL)
pm2 start "cargo run --release" --name ai-core-backend

# Start frontend
cd frontend
pm2 start server.js --name ai-core-frontend

# Save configuration
pm2 save
pm2 startup
```

## 📚 Documentation

- **API_DOCUMENTATION.md** - Complete API reference
- **TESTING_GUIDE.md** - Backend testing guide
- **frontend/README.md** - Frontend documentation
- **QUICKSTART.md** - Backend quick start
- **SUMMARY.md** - Implementation summary

## 🎯 Next Steps

1. ✅ Create some experiences
2. ✅ Try pattern recognition
3. ✅ Test personality updates
4. ✅ Make some decisions
5. ✅ Build your own features!

## 💡 Tips

1. **Keep both terminals visible**: Watch logs in real-time
2. **Use browser DevTools**: F12 for debugging
3. **Test incrementally**: Create → View → Search → Stats
4. **Check server logs**: `/interact` and `/reflect` show extra info
5. **Persistent storage**: Data saved to `data/memory.json`

## 🎉 You're All Set!

Your complete AI Core system with web interface is now running!

**Backend**: http://localhost:3000
**Frontend**: http://localhost:8080

Happy coding! 🚀

---

**Questions?** Check documentation or inspect the code!
**Issues?** See troubleshooting section above.
**Want more?** Extend with new features!
