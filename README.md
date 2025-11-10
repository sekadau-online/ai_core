# AI Core - Modular AI Memory & Pattern Recognition System

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Warnings](https://img.shields.io/badge/warnings-0-brightgreen)
![Errors](https://img.shields.io/badge/errors-0-brightgreen)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

Sistem AI Core yang robust dan modular dengan REST API endpoints dan authentication menggunakan Bearer token.

## 🚀 Fitur Utama

- **Memory Management**: Penyimpanan dan pengambilan pengalaman AI dengan pencarian
- **Pattern Recognition**: Analisis pola otomatis dari pengalaman yang tersimpan
- **Decision Making**: Pengambilan keputusan cerdas berdasarkan pengalaman dan pola
- **REST API**: 15+ API endpoints dengan authentication Bearer token
- **Persistent Storage**: Auto-save setiap 60 detik ke JSON file
- **Thread-Safe**: Menggunakan Arc<RwLock> untuk concurrent access yang aman
- **Personality System**: Sistem kepribadian AI yang dinamis (Big Five traits)
- **Dialog System**: Interaksi natural dengan pattern analysis
- **Reflection**: Introspeksi memori dengan timestamp tracking

## 📋 Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo
- WSL/Linux (untuk development)

## 🔧 Installation & Setup

### 1. Clone Repository
```bash
git clone https://github.com/sekadau-online/ai_core.git
cd ai_core
```

### 2. Setup Environment
Buat file `.env` di root folder:
```env
BEARER_TOKEN=rahasia_token_anda_yang_kuat_123456
HOST=127.0.0.1
PORT=3000
```

**⚠️ PENTING**: Gunakan token yang kuat dan rahasia untuk production!

### 3. Build Project
```bash
cargo build --release
```

### 4. Run Server
```bash
cargo run --release
```

Output:
```
🚀 Starting AI Core API
   Bearer Token configured: true
   Starting with fresh memory
   API listening on http://127.0.0.1:3000
   Use Bearer token in Authorization header

📝 Example request:
   curl -H 'Authorization: Bearer rahasia_token_anda' http://127.0.0.1:3000/health
```

Server akan berjalan di `http://127.0.0.1:3000` (atau sesuai konfigurasi di `.env`)

## 📡 API Endpoints

### Public Endpoints (No Authentication)

#### GET /
Root endpoint
```bash
curl http://localhost:3000/
```

#### GET /health
Health check
```bash
curl http://localhost:3000/health
```

### Protected Endpoints (Require Bearer Token)

**Note**: Semua endpoint berikut memerlukan header Authorization dengan Bearer token:
```
Authorization: Bearer your_secret_token_here
```

#### GET /experiences
Mendapatkan semua pengalaman

```bash
curl -H "Authorization: Bearer your_secret_token_here" \
     http://localhost:3000/experiences
```

#### GET /experiences/:id
Mendapatkan pengalaman berdasarkan ID

```bash
curl -H "Authorization: Bearer your_secret_token_here" \
     http://localhost:3000/experiences/1234567890-user
```

#### POST /experiences
Membuat pengalaman baru

```bash
curl -X POST \
     -H "Authorization: Bearer your_secret_token_here" \
     -H "Content-Type: application/json" \
     -d '{"content": "User bertanya tentang AI", "source": "user", "metadata": "important"}' \
     http://localhost:3000/experiences
```

#### GET /experiences/search?q=query
Mencari pengalaman

```bash
curl -H "Authorization: Bearer your_secret_token_here" \
     "http://localhost:3000/experiences/search?q=AI"
```

#### GET /stats
Mendapatkan statistik dan pola

```bash
curl -H "Authorization: Bearer your_secret_token_here" \
     http://localhost:3000/stats
```

#### GET /decision
Membuat keputusan berdasarkan pengalaman

```bash
curl -H "Authorization: Bearer your_secret_token_here" \
     http://localhost:3000/decision
```

#### GET /decision/query?q=query
Membuat keputusan untuk query tertentu

```bash
curl -H "Authorization: Bearer your_secret_token_here" \
     "http://localhost:3000/decision/query?q=cara+belajar+AI"
```

#### DELETE /memory/clear
Menghapus semua pengalaman

```bash
curl -X DELETE \
     -H "Authorization: Bearer your_secret_token_here" \
     http://localhost:3000/memory/clear
```

## 📊 Response Format

Semua response menggunakan format JSON standar:

```json
{
  "success": true,
  "data": { ... },
  "message": "Operation successful"
}
```

## 🏗️ Struktur Project

```
ai_core/
├── src/
│   ├── main.rs           # Entry point & server setup
│   ├── api.rs            # API handlers
│   ├── config.rs         # Configuration management
│   ├── middleware.rs     # Authentication middleware
│   ├── memory.rs         # Memory storage
│   ├── experience.rs     # Experience data structure
│   ├── pattern.rs        # Pattern recognition
│   ├── decision.rs       # Decision making
│   ├── dialog.rs         # Dialog interaction
│   └── personality.rs    # Personality traits
├── data/
│   └── memory.json       # Persisted memory (auto-created)
├── .env                  # Environment configuration
├── .env.example          # Example configuration
└── Cargo.toml           # Dependencies
```

## 🔒 Security

- Bearer token disimpan di file `.env` (tidak di-commit ke git)
- Authentication middleware memvalidasi semua protected endpoints
- CORS enabled untuk development (sesuaikan untuk production)

## 🛠️ Development

### Running in Development Mode
```bash
cargo run
```

### Running with Debug Logs
```bash
RUST_LOG=debug cargo run
```

### Build for Production
```bash
cargo build --release
./target/release/ai_core
```

## 📝 Example Usage Flow

1. Start server
2. Create experiences:
```bash
curl -X POST \
     -H "Authorization: Bearer ai_core_secret_token_2025" \
     -H "Content-Type: application/json" \
     -d '{"content": "Belajar tentang Rust programming", "source": "user"}' \
     http://localhost:3000/experiences
```

3. View statistics:
```bash
curl -H "Authorization: Bearer ai_core_secret_token_2025" \
     http://localhost:3000/stats
```

4. Make decisions:
```bash
curl -H "Authorization: Bearer ai_core_secret_token_2025" \
     http://localhost:3000/decision
```

## 🔄 Auto-Save

Memory secara otomatis disimpan ke `data/memory.json` setiap 60 detik.

## 📄 License

MIT License

## 🤝 Contributing

Contributions welcome! Please open an issue or submit a pull request.
