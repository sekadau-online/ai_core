# AI Core REST API Documentation

## Overview
AI Core menyediakan REST API untuk manajemen memori, analisis pattern, pengambilan keputusan, dan personality modeling.

## Authentication
Semua endpoint (kecuali public endpoints) memerlukan Bearer token di header:
```
Authorization: Bearer YOUR_TOKEN_HERE
```

Token dikonfigurasi di file `.env`:
```
BEARER_TOKEN=your_secret_token_here
```

## Base URL
```
http://localhost:3000
```

## Public Endpoints

### GET /
Informasi API dasar
```bash
curl http://localhost:3000/
```

### GET /health
Health check endpoint
```bash
curl http://localhost:3000/health
```

Response:
```json
{
  "success": true,
  "data": "AI Core is running",
  "message": "OK"
}
```

## Protected Endpoints

### 1. Experience Management

#### GET /experiences
Mengambil semua experiences
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/experiences
```

Response:
```json
{
  "success": true,
  "data": [
    {
      "id": "exp_123",
      "content": "Hello world",
      "source": "user",
      "timestamp": "2025-11-10T10:30:00Z",
      "metadata": null
    }
  ],
  "message": "Retrieved 1 experiences"
}
```

#### GET /experiences/:id
Mengambil experience berdasarkan ID
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/experiences/exp_123
```

#### POST /experiences
Membuat experience baru
```bash
curl -X POST \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "User berinteraksi dengan sistem",
    "source": "user",
    "metadata": "konteks tambahan"
  }' \
  http://localhost:3000/experiences
```

Response:
```json
{
  "success": true,
  "data": {
    "id": "exp_456",
    "content": "User berinteraksi dengan sistem",
    "source": "user",
    "timestamp": "2025-11-10T10:35:00Z",
    "metadata": "konteks tambahan"
  },
  "message": "Experience created successfully"
}
```

#### GET /experiences/search?q=query
Mencari experiences berdasarkan keyword
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  "http://localhost:3000/experiences/search?q=hello"
```

### 2. Pattern Analysis

#### GET /stats
Mendapatkan statistik dan top patterns
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/stats
```

Response:
```json
{
  "success": true,
  "data": {
    "total_experiences": 5,
    "total_patterns": 20,
    "top_patterns": [
      {
        "keyword": "user",
        "frequency": 10,
        "experience_count": 5
      }
    ]
  },
  "message": "Statistics retrieved"
}
```

#### GET /patterns/:keyword
Mendapatkan detail pattern untuk keyword tertentu
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/patterns/hello
```

Response:
```json
{
  "success": true,
  "data": {
    "keyword": "hello",
    "frequency": 3,
    "experience_ids": ["exp_1", "exp_2", "exp_3"],
    "related_experiences": [
      "Hello world",
      "Hello there",
      "Say hello"
    ]
  },
  "message": "Found pattern for keyword: hello"
}
```

#### POST /patterns/clear
Clear dan rebuild pattern cache
```bash
curl -X POST \
  -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/patterns/clear
```

### 3. Decision Making

#### GET /decision
Membuat keputusan berdasarkan seluruh memory
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/decision
```

Response:
```json
{
  "success": true,
  "data": {
    "action": "respond",
    "confidence": 0.85,
    "reasoning": "Based on 10 experiences and 5 patterns",
    "timestamp": "2025-11-10T10:40:00Z"
  },
  "message": "Decision made"
}
```

#### GET /decision/query?q=question
Membuat keputusan untuk query spesifik
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  "http://localhost:3000/decision/query?q=what+to+do"
```

### 4. AI Interaction

#### GET /interact
Analisis experiences dengan pattern recognizer
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/interact
```

Response:
```json
{
  "success": true,
  "data": {
    "analysis": "Analyzed 5 experiences",
    "experience_count": 5,
    "pattern_summary": [
      "user: 10 occurrences",
      "hello: 5 occurrences",
      "world: 3 occurrences"
    ]
  },
  "message": "Interaction completed"
}
```

### 5. Personality Management

#### POST /personality
Update personality berdasarkan input dan response
```bash
curl -X POST \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "input": "User bertanya dengan sopan",
    "response": "Terima kasih atas pertanyaannya"
  }' \
  http://localhost:3000/personality
```

Response:
```json
{
  "success": true,
  "data": {
    "curiosity": 0.6,
    "happiness": 0.7,
    "caution": 0.4,
    "dominant_trait": "happy",
    "influenced_response": "😊 Terima kasih atas pertanyaannya! Dengan senang hati saya membantu."
  },
  "message": "Personality updated"
}
```

### 6. Memory Reflection

#### GET /reflect
Mendapatkan refleksi terstruktur dari semua experiences
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/reflect
```

Response:
```json
{
  "success": true,
  "data": {
    "total_experiences": 5,
    "experiences": [
      {
        "id": "exp_1",
        "timestamp": "2025-11-10 10:30:00",
        "source": "user",
        "content": "Hello world"
      }
    ]
  },
  "message": "Reflected on 5 experiences"
}
```

### 7. Memory Management

#### DELETE /memory/clear
Menghapus semua experiences dari memory
```bash
curl -X DELETE \
  -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/memory/clear
```

Response:
```json
{
  "success": true,
  "data": "Memory cleared",
  "message": "All experiences have been deleted"
}
```

## Error Responses

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "message": "Missing or invalid Bearer token"
}
```

### 404 Not Found
```json
{
  "error": "Not Found",
  "message": "Resource not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal Server Error",
  "message": "Something went wrong"
}
```

## Complete Example Workflow

```bash
# 1. Set token
TOKEN="your_secret_token"

# 2. Check health
curl http://localhost:3000/health

# 3. Create experiences
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello AI", "source": "user"}' \
  http://localhost:3000/experiences

# 4. Get all experiences
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/experiences

# 5. Get statistics
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/stats

# 6. Interact with AI
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/interact

# 7. Make decision
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/decision

# 8. Update personality
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"input": "Be friendly", "response": "Hello!"}' \
  http://localhost:3000/personality

# 9. Reflect on memory
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/reflect
```

## Data Persistence

- Memory secara otomatis disimpan ke `data/memory.json` setiap 60 detik
- Memory dimuat otomatis saat server start
- Gunakan endpoint `/memory/clear` untuk menghapus semua data

## Configuration

File `.env`:
```env
BEARER_TOKEN=your_super_secret_token_here
HOST=127.0.0.1
PORT=3000
```

## Running the Server

```bash
# WSL/Linux
cargo run

# Atau dengan release mode
cargo run --release
```

Server akan berjalan di `http://127.0.0.1:3000`
