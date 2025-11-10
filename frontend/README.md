# AI Core Frontend

Web interface untuk AI Core API menggunakan Node.js + Express + EJS.

## 🚀 Quick Start

### 1. Install Dependencies
```bash
cd frontend
npm install
```

### 2. Configure Environment
Edit file `.env`:
```env
BACKEND_URL=http://127.0.0.1:3000
BEARER_TOKEN=rahasia_token_anda_yang_kuat_123456
PORT=8080
```

### 3. Start Frontend Server
```bash
npm start
```

Atau untuk development dengan auto-reload:
```bash
npm run dev
```

### 4. Open Browser
Navigate to: `http://localhost:8080`

## 📋 Prerequisites

- Node.js 14+ installed
- AI Core backend server running on port 3000
- Bearer token from backend `.env` file

## 🎯 Features

### 📝 Pages Available

1. **Home** (`/`) - Dashboard dengan status API dan feature cards
2. **Experiences** (`/experiences`) - View semua experiences
3. **Create** (`/create`) - Form create experience baru
4. **Search** (`/search`) - Search experiences by keyword
5. **Statistics** (`/stats`) - View patterns dan analytics
6. **Personality** (`/personality`) - Update AI personality traits
7. **Decision** (`/decision`) - AI decision making
8. **Interact** (`/interact`) - Interact dengan AI system
9. **Reflect** (`/reflect`) - Memory reflection timeline

### 🎨 UI Features

- Responsive design (mobile-friendly)
- Modern gradient UI
- Real-time API status indicator
- Toast notifications
- Modal dialogs
- Loading states
- Error handling

## 📂 Project Structure

```
frontend/
├── server.js              # Express server
├── package.json           # Dependencies
├── .env                   # Configuration
├── views/                 # EJS templates
│   ├── index.ejs          # Home page
│   ├── experiences.ejs    # Experiences list
│   ├── create.ejs         # Create form
│   ├── search.ejs         # Search page
│   ├── stats.ejs          # Statistics
│   ├── personality.ejs    # Personality
│   ├── decision.ejs       # Decision
│   ├── interact.ejs       # Interact
│   ├── reflect.ejs        # Reflect
│   └── partials/          # Reusable components
│       ├── navbar.ejs
│       └── footer.ejs
└── public/                # Static assets
    ├── css/
    │   └── style.css      # Main stylesheet
    └── js/
        └── main.js        # Client-side JS
```

## 🔌 API Integration

Frontend menggunakan axios untuk berkomunikasi dengan backend:

```javascript
const apiClient = axios.create({
    baseURL: BACKEND_URL,
    headers: {
        'Authorization': `Bearer ${BEARER_TOKEN}`,
        'Content-Type': 'application/json'
    }
});
```

Semua API calls di-proxy melalui Express server untuk security.

## 🛠️ Development

### Start with Auto-Reload
```bash
npm run dev
```

### Available Scripts

- `npm start` - Start production server
- `npm run dev` - Start development server with nodemon

### Adding New Pages

1. Create EJS template in `views/`
2. Add route in `server.js`
3. Add navigation link in `partials/navbar.ejs`
4. Style in `public/css/style.css`

## 🎨 Styling Guide

### Color Scheme
- Primary: `#3b82f6` (Blue)
- Secondary: `#8b5cf6` (Purple)
- Success: `#10b981` (Green)
- Danger: `#ef4444` (Red)
- Warning: `#f59e0b` (Orange)

### CSS Classes

- `.btn` - Button base
- `.btn-primary` - Primary button
- `.btn-secondary` - Secondary button
- `.btn-danger` - Danger button
- `.alert-success` - Success message
- `.alert-error` - Error message
- `.info-card` - Information card
- `.empty-state` - Empty state message

## 🔒 Security

- Bearer token stored in `.env` (not in code)
- API calls proxied through backend
- CORS handled by backend
- No direct client-to-API calls

## 🐛 Troubleshooting

### Issue: "Cannot connect to backend"
**Solution**: Ensure backend is running on port 3000
```bash
# Terminal 1: Start backend
cd ..
cargo run

# Terminal 2: Start frontend
cd frontend
npm start
```

### Issue: "401 Unauthorized"
**Solution**: Check token in `.env` matches backend token

### Issue: "Port 8080 already in use"
**Solution**: Change PORT in `.env`

## 📸 Screenshots

### Home Dashboard
- API status indicator
- Feature cards
- Quick actions

### Experiences Page
- List all experiences
- Create new button
- Clear memory button

### Statistics Page
- Total counters
- Top patterns list
- Pattern details modal

### Personality Page
- Trait visualization
- Progress bars
- Emoji indicators

## 🚀 Deployment

### Production Build

1. Set environment variables
2. Start with PM2 or similar:

```bash
npm install -g pm2
pm2 start server.js --name ai-core-frontend
pm2 save
pm2 startup
```

### Docker (Optional)

```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install --production
COPY . .
EXPOSE 8080
CMD ["node", "server.js"]
```

## 📚 Dependencies

- **express** - Web framework
- **axios** - HTTP client
- **ejs** - Template engine
- **dotenv** - Environment variables
- **body-parser** - Request parsing
- **cookie-parser** - Cookie handling

## 🎓 Learning Resources

- Express.js: https://expressjs.com/
- EJS Templates: https://ejs.co/
- Axios: https://axios-http.com/

## 📝 License

MIT

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Test thoroughly
5. Submit pull request

---

**Built with ❤️ using Node.js + Express + EJS**
