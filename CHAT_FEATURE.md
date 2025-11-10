# 💬 Live Chat Feature - AI Core

## Overview

Fitur Live Chat adalah sistem percakapan interaktif yang memungkinkan pengguna berkomunikasi dengan AI yang memiliki akses penuh ke semua memori dan pengalaman yang tersimpan. AI memberikan respons yang context-aware berdasarkan data yang dimiliki.

## Features

### 1. **Context-Aware Chat** 🧠
- AI menganalisis pertanyaan Anda dan mencari konteks relevan dari memori
- Setiap respons menunjukkan berapa banyak konteks yang digunakan
- Pattern recognition untuk memberikan insights yang lebih baik

### 2. **Document Upload** 📄
- Upload file dalam format:
  - **TXT** - Plain text documents
  - **JSON** - Structured data (otomatis di-parse)
  - **CSV** - Tabular data
- File langsung ditambahkan ke memori AI
- AI dapat menggunakan informasi dari file untuk menjawab pertanyaan

### 3. **Export Conversation** 💾
- Export percakapan dalam 4 format:
  - **JSON** - Machine-readable format
  - **TXT** - Plain text format
  - **Markdown** - Formatted with emojis
  - **HTML** - Styled web page
- Download langsung ke komputer Anda

### 4. **cURL Execution** 🔌
- Execute curl commands untuk memanggil API eksternal
- Security: Hanya GET requests yang diizinkan
- Hasil ditampilkan di UI dan di chat

## API Endpoints

### Send Chat Message
```bash
POST /chat/send
Authorization: Bearer {token}
Content-Type: application/json

{
  "content": "Apa yang kamu tahu tentang AI?",
  "session_id": "optional-session-id"
}

Response:
{
  "success": true,
  "data": {
    "session_id": "session_uuid",
    "message": {
      "id": "msg_uuid",
      "role": "assistant",
      "content": "Response text...",
      "timestamp": "2025-11-10T10:30:00Z",
      "context_used": ["exp_id1", "exp_id2"]
    },
    "context_count": 2
  }
}
```

### Get Chat History
```bash
GET /chat/history/{session_id}
Authorization: Bearer {token}

Response:
{
  "success": true,
  "data": {
    "id": "session_uuid",
    "messages": [...],
    "created_at": "2025-11-10T10:00:00Z",
    "updated_at": "2025-11-10T10:30:00Z"
  }
}
```

### Upload Document
```bash
POST /chat/upload
Authorization: Bearer {token}
Content-Type: application/json

{
  "filename": "data.txt",
  "content": "file content here...",
  "filetype": "txt"
}

Response:
{
  "success": true,
  "data": {
    "processed": true,
    "text": "extracted content...",
    "added_to_memory": true
  }
}
```

### Export Chat Session
```bash
GET /chat/export?session_id={id}&format={json|txt|markdown|html}
Authorization: Bearer {token}

Response:
{
  "success": true,
  "data": "exported content as string"
}
```

### Execute cURL Command
```bash
POST /chat/curl
Authorization: Bearer {token}
Content-Type: application/json

{
  "command": "curl https://api.example.com/data"
}

Response:
{
  "success": true,
  "data": {
    "executed": true,
    "output": "curl output..."
  }
}
```

### List All Sessions
```bash
GET /chat/sessions
Authorization: Bearer {token}

Response:
{
  "success": true,
  "data": ["session_id1", "session_id2", ...]
}
```

### Clear Session
```bash
DELETE /chat/sessions/{session_id}
Authorization: Bearer {token}

Response:
{
  "success": true,
  "message": "Session cleared"
}
```

## How It Works

### Chat Processing Flow

```
User Input
    ↓
Extract Keywords
    ↓
Search Memory for Relevant Experiences
    ↓
Analyze Patterns
    ↓
Generate Context-Aware Response
    ↓
Return Response with Context Info
```

### Document Processing Flow

```
Upload File
    ↓
Detect File Type (TXT/JSON/CSV)
    ↓
Extract Content
    ├─ TXT: Direct content
    ├─ JSON: Recursive string extraction
    └─ CSV: Parse rows
    ↓
Add to Memory as Experience
    ↓
Available for Chat Context
```

## Usage Examples

### Example 1: Simple Chat
```javascript
// User asks
"Apa itu machine learning?"

// AI searches memory for relevant experiences
// Finds 3 experiences about ML
// Returns context-aware response with insights
```

### Example 2: Upload & Query
```javascript
// 1. Upload document about AI
POST /chat/upload
{
  "filename": "ai_basics.txt",
  "content": "Artificial Intelligence is...",
  "filetype": "txt"
}

// 2. Ask question about the document
POST /chat/send
{
  "content": "Jelaskan tentang AI berdasarkan dokumen"
}

// AI uses the uploaded document as context
```

### Example 3: Export Conversation
```javascript
// After chatting, export as Markdown
GET /chat/export?session_id=abc123&format=markdown

// Downloads: chat_abc123.markdown
// Contains formatted conversation with emojis
```

### Example 4: Execute cURL
```javascript
POST /chat/curl
{
  "command": "curl https://api.github.com/users/github"
}

// Returns GitHub user info
// Result shown in chat and curl output section
```

## Frontend Integration

### Initialize Chat
```javascript
let currentSessionId = null;

async function sendMessage() {
    const response = await fetch(`${API_URL}/chat/send`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${BEARER_TOKEN}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            content: message,
            session_id: currentSessionId
        })
    });
    
    const result = await response.json();
    currentSessionId = result.data.session_id;
}
```

### Upload File
```javascript
const content = await readFileContent(file);
const fileType = file.name.split('.').pop();

await fetch(`${API_URL}/chat/upload`, {
    method: 'POST',
    headers: {
        'Authorization': `Bearer ${BEARER_TOKEN}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        filename: file.name,
        content: content,
        filetype: fileType
    })
});
```

## Security Features

1. **Bearer Token Authentication**: All endpoints require valid token
2. **cURL Restrictions**: Only GET requests allowed
3. **Session Isolation**: Each session is independent
4. **Input Validation**: All inputs are validated before processing

## Performance Considerations

1. **Memory Search**: Optimized keyword extraction and matching
2. **Pattern Analysis**: Incremental pattern building
3. **Session Storage**: In-memory storage using `lazy_static`
4. **File Processing**: Efficient text extraction without full parsing

## Future Enhancements

- [ ] WebSocket support for real-time streaming
- [ ] Voice input/output
- [ ] Image upload and analysis
- [ ] Multi-language support
- [ ] Conversation templates
- [ ] AI personality selection per session
- [ ] Markdown rendering in chat
- [ ] Code syntax highlighting
- [ ] Session persistence to database
- [ ] Rate limiting per session

## Troubleshooting

### Issue: "Session not found"
**Solution**: Session might have been cleared. Start a new conversation.

### Issue: "Failed to upload file"
**Solution**: Check file format (TXT, JSON, CSV only) and file size.

### Issue: "No context found"
**Solution**: Add more experiences or documents to memory first.

### Issue: "cURL execution failed"
**Solution**: Ensure curl command starts with "curl" and uses GET only.

## Testing

### Test Chat API
```bash
curl -X POST http://localhost:3000/chat/send \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Halo, apa kabar?"
  }'
```

### Test Document Upload
```bash
curl -X POST http://localhost:3000/chat/upload \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456" \
  -H "Content-Type: application/json" \
  -d '{
    "filename": "test.txt",
    "content": "This is a test document about AI and machine learning.",
    "filetype": "txt"
  }'
```

### Test Export
```bash
curl -X GET "http://localhost:3000/chat/export?session_id=YOUR_SESSION&format=json" \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456"
```

## Architecture

```
Frontend (Port 8080)
├── chat.ejs          # Chat UI
├── server.js         # Express routes
└── public/css/       # Styling

Backend (Port 3000)
├── src/chat.rs       # Chat logic
│   ├── ChatMessage
│   ├── ChatSession
│   ├── ChatProcessor
│   ├── DocumentProcessor
│   └── ChatExporter
├── src/api.rs        # API handlers
└── src/main.rs       # Route definitions

Data Flow
User Input → Frontend → Backend API → Chat Processor → Memory Search → Pattern Analysis → Response
```

## Modules

### `ChatMessage`
- Struktur untuk menyimpan pesan individual
- Menyimpan role (user/assistant), content, timestamp, context

### `ChatSession`
- Manajemen sesi percakapan
- Menyimpan semua pesan dalam satu sesi
- Auto-update timestamp

### `ChatProcessor`
- Process pesan dengan context-awareness
- Ekstraksi keywords dari input
- Generate respons berdasarkan memori dan pattern

### `DocumentProcessor`
- Proses file upload (TXT, JSON, CSV)
- Ekstraksi konten dari berbagai format
- Konversi ke format yang bisa disimpan di memori

### `ChatExporter`
- Export percakapan ke berbagai format
- Support JSON, TXT, Markdown, HTML
- Formatting otomatis dengan timestamp dan metadata

## License

Part of AI Core project - Modular, Robust, Consistent AI System with REST API.

---

**Created**: November 2025
**Status**: ✅ Production Ready
**Version**: 1.0.0
