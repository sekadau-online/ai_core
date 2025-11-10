# 🎓 API Learning Feature - Full CRUD System

## Overview

Fitur API Learning adalah sistem CRUD lengkap yang memungkinkan AI untuk **belajar dari hasil API calls**. Setiap HTTP request yang dieksekusi dapat disimpan sebagai learning record, diberi tag, dan digunakan sebagai konteks untuk percakapan selanjutnya.

## 🆕 Perubahan dari Versi Sebelumnya

### Sebelum (cURL Only):
- ❌ Hanya bisa GET request
- ❌ Tidak ada penyimpanan hasil
- ❌ Tidak ada learning mechanism
- ❌ Hasil tidak bisa digunakan untuk konteks

### Sekarang (Full CRUD Learning System):
- ✅ Support semua HTTP methods (GET, POST, PUT, DELETE)
- ✅ Setiap request disimpan sebagai learning record
- ✅ Auto-tagging dari URL dan response
- ✅ CRUD lengkap untuk learning records
- ✅ Search dan filter berdasarkan tag/URL
- ✅ Otomatis ditambahkan ke memory AI
- ✅ Bisa digunakan sebagai konteks dalam chat

## 📋 Features

### 1. **Execute HTTP Request** 🚀
Execute HTTP request dengan method apapun dan simpan hasilnya:
```json
POST /api-learning/execute
{
  "method": "GET",
  "url": "https://api.github.com/users/github",
  "body": null,
  "headers": [["Content-Type", "application/json"]],
  "save_to_memory": true
}
```

### 2. **Get All Learning Records** 📚
Ambil semua learning records yang tersimpan:
```bash
GET /api-learning/records
```

### 3. **Get Learning Record by ID** 🔍
Ambil detail learning record spesifik:
```bash
GET /api-learning/records/{id}
```

### 4. **Update Learning Record** ✏️
Update tags dan summary:
```json
POST /api-learning/records/{id}
{
  "tags": ["api", "github", "users"],
  "summary": "GitHub user profile API"
}
```

### 5. **Delete Learning Record** 🗑️
Hapus learning record:
```bash
DELETE /api-learning/records/{id}
```

### 6. **Search Learning Records** 🔎
Cari berdasarkan URL, tag, atau summary:
```bash
GET /api-learning/search?q=github
```

### 7. **Clear All Records** 🧹
Hapus semua learning records:
```bash
DELETE /api-learning/clear
```

## 🏗️ Data Structure

### ApiLearningRecord
```rust
pub struct ApiLearningRecord {
    pub id: String,              // Unique ID: api_{uuid}
    pub method: String,          // GET, POST, PUT, DELETE
    pub url: String,             // Full URL
    pub request_body: Option<String>,  // Request body jika ada
    pub response_body: String,   // Response dari API
    pub status_code: u16,        // HTTP status code
    pub learned_at: DateTime,    // Timestamp
    pub tags: Vec<String>,       // Auto-extracted tags
    pub summary: String,         // Auto-generated summary
}
```

### Auto-Tagging System
Tags diekstrak otomatis dari:
1. **Domain**: `api.github.com`
2. **Path segments**: `users`, `repos`
3. **Status code category**: `2xx`, `4xx`, `5xx`

Contoh:
```
URL: https://api.github.com/users/github/repos
Tags: ["api.github.com", "users", "github"]
```

### Auto-Summary Generation
Summary dibuat otomatis berdasarkan:
- Status code (Success/Client Error/Server Error)
- URL
- Timestamp

Contoh:
```
"Success - https://api.github.com/users/github (200)"
```

## 📡 API Endpoints Details

### 1. Execute HTTP Request

**Endpoint:** `POST /api-learning/execute`

**Request:**
```json
{
  "method": "POST",
  "url": "https://jsonplaceholder.typicode.com/posts",
  "body": "{\"title\":\"Test\",\"body\":\"Content\",\"userId\":1}",
  "headers": [
    ["Content-Type", "application/json"]
  ],
  "save_to_memory": true
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "success": true,
    "status": 201,
    "body": "{\"id\": 101, \"title\": \"Test\", ...}",
    "learning_record_id": "api_a1b2c3d4-..."
  },
  "message": "HTTP request executed"
}
```

**What Happens:**
1. HTTP request dieksekusi
2. Response disimpan sebagai `ApiLearningRecord`
3. Auto-tagging dari URL
4. Auto-summary generation
5. Ditambahkan ke memory AI sebagai `Experience`
6. Bisa digunakan dalam chat context

### 2. Get All Learning Records

**Endpoint:** `GET /api-learning/records`

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "api_uuid1",
      "method": "GET",
      "url": "https://api.github.com/users/github",
      "request_body": null,
      "response_body": "{...}",
      "status_code": 200,
      "learned_at": "2025-11-10T10:00:00Z",
      "tags": ["api.github.com", "users"],
      "summary": "Success - https://api.github.com/users/github (200)"
    }
  ],
  "message": "Retrieved 15 learning records"
}
```

### 3. Update Learning Record

**Endpoint:** `POST /api-learning/records/{id}`

**Request:**
```json
{
  "tags": ["github", "api", "users", "profile"],
  "summary": "GitHub user profile data - verified and tested"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "api_uuid1",
    "tags": ["github", "api", "users", "profile"],
    "summary": "GitHub user profile data - verified and tested",
    ...
  },
  "message": "Learning record updated"
}
```

### 4. Search Learning Records

**Endpoint:** `GET /api-learning/search?q=github`

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "api_uuid1",
      "url": "https://api.github.com/users/github",
      "tags": ["api.github.com", "users"],
      ...
    },
    {
      "id": "api_uuid2",
      "url": "https://api.github.com/repos/...",
      "tags": ["api.github.com", "repos"],
      ...
    }
  ],
  "message": "Found 2 matching records"
}
```

## 💡 Use Cases

### Use Case 1: Learning API Patterns
```javascript
// 1. Execute berbagai GitHub API calls
POST /api-learning/execute { "method": "GET", "url": "https://api.github.com/users/torvalds" }
POST /api-learning/execute { "method": "GET", "url": "https://api.github.com/users/gvanrossum" }

// 2. Search untuk melihat pola
GET /api-learning/search?q=github

// 3. AI sekarang tahu tentang GitHub API structure
POST /chat/send { "content": "Bagaimana struktur GitHub user API?" }
// AI akan menjawab berdasarkan learning records
```

### Use Case 2: API Documentation Builder
```javascript
// 1. Execute semua endpoints dari suatu API
POST /api-learning/execute (multiple calls ke berbagai endpoints)

// 2. Update tags untuk kategorisasi
POST /api-learning/records/{id} { "tags": ["auth", "public", "v1"] }

// 3. Export semua records untuk dokumentasi
GET /api-learning/records
// Generate documentation dari learning records
```

### Use Case 3: API Testing & Monitoring
```javascript
// 1. Execute API call dan simpan baseline
POST /api-learning/execute { "url": "https://api.example.com/health" }

// 2. Execute lagi nanti untuk compare
POST /api-learning/execute { "url": "https://api.example.com/health" }

// 3. Search untuk melihat history
GET /api-learning/search?q=health
// Lihat perubahan status code atau response
```

### Use Case 4: Context-Aware Chat
```javascript
// 1. Upload API documentation sebagai learning
POST /api-learning/execute (ke API endpoints)

// 2. Tanya AI tentang API
POST /chat/send { "content": "Jelaskan cara menggunakan endpoint /users" }

// 3. AI menggunakan learning records sebagai konteks
// Response akan berdasarkan actual API calls yang disimpan
```

## 🖥️ Frontend Integration

### Execute HTTP Request
```javascript
async function executeHttpRequest() {
    const response = await fetch(`${API_URL}/api-learning/execute`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${BEARER_TOKEN}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            method: 'GET',
            url: 'https://api.example.com/data',
            save_to_memory: true
        })
    });
    
    const result = await response.json();
    console.log('Learning Record ID:', result.data.learning_record_id);
}
```

### View Learning Records
```javascript
async function viewLearningRecords() {
    const response = await fetch(`${API_URL}/api-learning/records`, {
        headers: { 'Authorization': `Bearer ${BEARER_TOKEN}` }
    });
    
    const result = await response.json();
    result.data.forEach(record => {
        console.log(`${record.method} ${record.url} - ${record.status_code}`);
        console.log('Tags:', record.tags.join(', '));
    });
}
```

### Search Records
```javascript
async function searchLearning(query) {
    const response = await fetch(
        `${API_URL}/api-learning/search?q=${encodeURIComponent(query)}`,
        { headers: { 'Authorization': `Bearer ${BEARER_TOKEN}` } }
    );
    
    const result = await response.json();
    return result.data; // Array of matching records
}
```

## 🔒 Security Features

1. **Bearer Token Required**: Semua endpoints memerlukan authentication
2. **URL Validation**: Hanya URL dengan http:// atau https://
3. **Input Sanitization**: Semua input divalidasi sebelum processing
4. **No Command Injection**: Tidak ada shell command execution
5. **Memory Safety**: Rust's memory safety guarantees

## 📊 Performance Considerations

### Storage
- In-memory storage menggunakan `HashMap`
- Thread-safe dengan `Arc<RwLock>`
- O(1) access by ID
- O(n) search (future: dapat ditingkatkan dengan indexing)

### Scalability
```rust
// Current: In-memory storage
static ref API_LEARNING_RECORDS: Arc<RwLock<HashMap<String, ApiLearningRecord>>>

// Future: Database integration
// - PostgreSQL untuk persistence
// - Redis untuk caching
// - Elasticsearch untuk full-text search
```

## 🧪 Testing Examples

### Test Execute & Save
```bash
curl -X POST http://localhost:3000/api-learning/execute \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456" \
  -H "Content-Type: application/json" \
  -d '{
    "method": "GET",
    "url": "https://jsonplaceholder.typicode.com/posts/1",
    "save_to_memory": true
  }'
```

### Test Get All Records
```bash
curl http://localhost:3000/api-learning/records \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456"
```

### Test Search
```bash
curl "http://localhost:3000/api-learning/search?q=posts" \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456"
```

### Test Update
```bash
curl -X POST http://localhost:3000/api-learning/records/api_abc123 \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456" \
  -H "Content-Type: application/json" \
  -d '{
    "tags": ["api", "posts", "example"],
    "summary": "JSONPlaceholder post API - test endpoint"
  }'
```

### Test Delete
```bash
curl -X DELETE http://localhost:3000/api-learning/records/api_abc123 \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456"
```

## 📈 Future Enhancements

- [ ] **Persistence**: Save learning records to database
- [ ] **Analytics**: Visualize API usage patterns
- [ ] **Auto-retry**: Retry failed requests with exponential backoff
- [ ] **Rate limiting**: Prevent API abuse
- [ ] **Webhook support**: Trigger actions on specific responses
- [ ] **Response comparison**: Compare responses over time
- [ ] **API versioning**: Track API version changes
- [ ] **Cost tracking**: Monitor API costs
- [ ] **SLA monitoring**: Track response times and availability
- [ ] **Mock server**: Generate mock responses from learning records

## 🎯 Benefits

### For AI Learning
- ✅ Contextual understanding dari actual API calls
- ✅ Real-world data untuk training
- ✅ Pattern recognition dari API responses
- ✅ Dapat memberikan contoh konkret dalam chat

### For Development
- ✅ API testing integrated
- ✅ Documentation otomatis dari actual calls
- ✅ History tracking untuk debugging
- ✅ Reusable test cases

### For Monitoring
- ✅ Track API availability
- ✅ Monitor response times
- ✅ Detect API changes
- ✅ Alert on errors

## 🔄 Integration dengan Chat

Learning records otomatis tersedia sebagai konteks dalam chat:

```javascript
// User bertanya
"Bagaimana cara menggunakan GitHub API?"

// AI mencari learning records dengan tag "github"
// Menemukan 5 records tentang GitHub API
// Menggunakan records tersebut sebagai konteks

// AI menjawab berdasarkan actual API calls:
"Berdasarkan 5 API calls yang saya pelajari, GitHub API memiliki struktur:
1. GET /users/{username} - mendapat profil user
2. GET /users/{username}/repos - mendapat repositories
..."
```

## 🎓 Learning Flow

```
HTTP Request → Execute
    ↓
Create ApiLearningRecord
    ↓
Auto-tag dari URL
    ↓
Auto-generate summary
    ↓
Save to HashMap
    ↓
Add to Memory as Experience
    ↓
Available in Chat Context
```

---

**Status**: ✅ Production Ready  
**Version**: 2.0.0 (Upgraded from cURL-only to Full CRUD)  
**Created**: November 2025  
**Last Updated**: November 2025

