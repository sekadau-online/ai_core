# 🤖 Ollama Integration - Local AI Power

## Overview

Integrasi Ollama memungkinkan AI Core menggunakan **Local Large Language Models** untuk memberikan respons chat yang lebih canggih, natural, dan context-aware tanpa perlu API eksternal atau biaya tambahan.

## 🎯 Why Ollama?

### Before (Fallback Responses):
```text
User: "Jelaskan tentang machine learning"
AI: "Berdasarkan 3 pengalaman relevan yang saya temukan:
1. content dari experience...
2. content dari experience...
```

### After (Ollama-Powered):
```text
User: "Jelaskan tentang machine learning"
AI: "Machine learning adalah cabang dari artificial intelligence yang memungkinkan sistem untuk belajar dan meningkatkan performa dari pengalaman tanpa diprogramkan secara eksplisit. Berdasarkan konteks yang saya miliki, machine learning melibatkan algoritma yang dapat mengidentifikasi pola dalam data dan membuat prediksi... [natural, comprehensive response]"
```

## 📦 Setup Ollama

### 1. Install Ollama

**Linux/WSL:**
```bash
curl https://ollama.ai/install.sh | sh
```

**Windows:**
Download dari: https://ollama.ai/download

**macOS:**
```bash
brew install ollama
```

### 2. Pull Model

```bash
# Pull default model (Llama 2)
ollama pull llama2

# Or other models:
ollama pull mistral
ollama pull codellama
ollama pull neural-chat
ollama pull phi
```

### 3. Start Ollama Server

```bash
# Start Ollama service
ollama serve

# Output:
# Ollama is running on http://localhost:11434
```

### 4. Test Ollama

```bash
# Test dengan curl
curl http://localhost:11434/api/tags

# Test dengan ollama CLI
ollama run llama2 "Hello, who are you?"
```

## ⚙️ Configuration

### Environment Variables (.env)

```bash
# Ollama Configuration
OLLAMA_URL=http://localhost:11434    # Ollama server URL
OLLAMA_MODEL=llama2                  # Model name to use
OLLAMA_ENABLED=true                  # Enable/disable Ollama
```

### Available Models

| Model | Size | Best For |
|-------|------|----------|
| `llama2` | 7B | General conversation |
| `mistral` | 7B | Instruction following |
| `codellama` | 7B-34B | Code generation |
| `neural-chat` | 7B | Chat & assistant |
| `phi` | 2.7B | Fast responses (lighter) |
| `orca-mini` | 3B | Efficient reasoning |
| `vicuna` | 7B-13B | Natural conversation |

### Enable/Disable Ollama

**Enable** (use Ollama for AI responses):
```bash
OLLAMA_ENABLED=true
```

**Disable** (use fallback responses):
```bash
OLLAMA_ENABLED=false
```

## 🔧 Implementation Details

### Backend Architecture

```rust
// src/ollama.rs
pub struct OllamaClient {
    url: String,
    model: String,
    enabled: bool,
}

impl OllamaClient {
    pub async fn generate(&self, prompt: &str) -> Result<String, String>
    pub async fn generate_with_context(&self, input: &str, context: &[String]) -> Result<String, String>
    pub async fn health_check(&self) -> bool
    pub async fn list_models(&self) -> Result<Vec<String>, String>
}
```

### Chat Processing Flow

```
User Message
    ↓
Extract Keywords
    ↓
Search Memory for Context
    ↓
Build Prompt with Context
    ↓
    ├──→ Ollama Enabled?
    │    ├── YES → Send to Ollama
    │    │         ↓
    │    │      Generate AI Response
    │    │         ↓
    │    │      Return Natural Response
    │    │
    │    └── NO → Use Fallback
    │              ↓
    │           Pattern-Based Response
    ↓
Return to User
```

### Request Structure

```json
POST http://localhost:11434/api/generate
{
  "model": "llama2",
  "prompt": "Context: ...\n\nUser: ...\n\nAssistant:",
  "stream": false
}
```

### Response Structure

```json
{
  "response": "Generated text here...",
  "done": true
}
```

## 📊 Features

### 1. **Context-Aware Generation**

Ollama receives context from memory:
```rust
let context_texts: Vec<String> = experiences
    .iter()
    .map(|e| format!("- {} (from {})", e.content, e.source))
    .collect();

ollama.generate_with_context(user_input, &context_texts).await
```

### 2. **Fallback Mechanism**

If Ollama fails, system automatically falls back:
```rust
match ollama.generate_with_context(input, context).await {
    Ok(response) => response,
    Err(e) => {
        tracing::warn!("Ollama failed: {}. Using fallback.", e);
        Self::generate_default_response(input)
    }
}
```

### 3. **Health Monitoring**

Server checks Ollama on startup:
```rust
if ollama_client.health_check().await {
    tracing::info!("✅ Ollama is running");
    // List available models
    let models = ollama_client.list_models().await?;
    tracing::info!("Available models: {}", models.join(", "));
} else {
    tracing::warn!("⚠️  Ollama not accessible");
}
```

### 4. **Timeout Protection**

Requests have 120s timeout:
```rust
client.post(&endpoint)
    .json(&request_body)
    .timeout(Duration::from_secs(120))
    .send()
    .await
```

## 🚀 Usage

### Start Server with Ollama

```bash
# 1. Start Ollama
ollama serve

# 2. Enable in .env
OLLAMA_ENABLED=true

# 3. Start AI Core
cargo run

# Output:
# 🚀 Starting AI Core API
#    Ollama enabled: true
#    Ollama URL: http://localhost:11434
#    Ollama Model: llama2
#    Checking Ollama connection...
#    ✅ Ollama is running and accessible
#    Available models: llama2, mistral, codellama
```

### Test Chat with Ollama

```bash
curl -X POST http://localhost:3000/chat/send \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Jelaskan tentang artificial intelligence"
  }'
```

### Response with Ollama:
```json
{
  "success": true,
  "data": {
    "message": {
      "content": "Artificial Intelligence (AI) adalah bidang ilmu komputer yang berfokus pada pengembangan sistem yang dapat melakukan tugas-tugas yang biasanya memerlukan kecerdasan manusia. Ini mencakup pembelajaran, penalaran, pemecahan masalah, persepsi, dan pemahaman bahasa..."
    }
  }
}
```

## 📈 Performance

### Response Times

| Scenario | Without Ollama | With Ollama (7B) |
|----------|---------------|------------------|
| Simple query | ~10ms | ~2-5s |
| With context (5 exp) | ~20ms | ~3-8s |
| Complex reasoning | ~15ms | ~5-15s |

### Resource Usage

| Model | RAM Usage | VRAM (GPU) |
|-------|-----------|------------|
| llama2 (7B) | ~8GB | ~6GB (optional) |
| mistral (7B) | ~8GB | ~6GB (optional) |
| phi (2.7B) | ~4GB | ~3GB (optional) |

**Note**: Ollama can run on CPU only, but GPU significantly improves speed.

## 🔍 Testing

### Check Ollama Status

```bash
# Health check
curl http://localhost:11434/api/tags

# List models
ollama list

# Test generation
curl http://localhost:11434/api/generate -d '{
  "model": "llama2",
  "prompt": "What is AI?",
  "stream": false
}'
```

### Test Integration

```bash
# Test with Ollama enabled
curl -X POST http://localhost:3000/chat/send \
  -H "Authorization: Bearer TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "test"}'

# Check logs for:
# - "Sending request to Ollama"
# - "Ollama response length: X chars"
# - Or "Using fallback" if Ollama fails
```

## 🛠️ Troubleshooting

### Issue: "Failed to connect to Ollama"

**Cause**: Ollama server not running

**Solution**:
```bash
# Check if Ollama is running
ps aux | grep ollama

# Start Ollama
ollama serve

# Or on Windows:
# Start "Ollama" application
```

### Issue: "Model not found"

**Cause**: Specified model not pulled

**Solution**:
```bash
# List available models
ollama list

# Pull the model
ollama pull llama2

# Or change model in .env
OLLAMA_MODEL=mistral  # Use different model
```

### Issue: "Timeout after 120s"

**Cause**: Model too large or complex prompt

**Solution**:
1. Use smaller model: `phi` or `orca-mini`
2. Reduce context size
3. Enable GPU acceleration
4. Increase timeout in code

### Issue: "Ollama returns empty response"

**Cause**: Model error or inappropriate prompt

**Solution**:
- Check Ollama logs: `ollama logs`
- Test model directly: `ollama run llama2 "test"`
- Try different model

## 🎨 Customization

### Custom Prompts

Edit `src/ollama.rs`:
```rust
pub async fn generate_with_context(
    &self,
    user_input: &str,
    context: &[String],
) -> Result<String, String> {
    let prompt = format!(
        "You are a helpful AI assistant with access to memory.\n\
        Context:\n{}\n\n\
        User: {}\n\n\
        Assistant:",
        context.join("\n"),
        user_input
    );
    
    self.generate(&prompt).await
}
```

### Model-Specific Settings

```rust
// For code-focused responses
OLLAMA_MODEL=codellama

// For Indonesian language
// Fine-tune prompt or use multilingual model
```

## 📊 Comparison

| Feature | Fallback | Ollama |
|---------|----------|--------|
| Response Quality | Basic | Advanced |
| Natural Language | Limited | Excellent |
| Reasoning | Pattern-based | Contextual |
| Creativity | Low | High |
| Speed | Fast (<50ms) | Slower (2-15s) |
| Cost | Free | Free (local) |
| Privacy | Complete | Complete |
| Internet Required | No | No |
| Setup | None | Install + Pull model |

## 🔐 Security & Privacy

### Advantages:
- ✅ **100% Local**: No data sent to external servers
- ✅ **Privacy**: All conversations stay on your machine
- ✅ **Offline**: Works without internet
- ✅ **No API Keys**: No costs or rate limits
- ✅ **Full Control**: You own the model and data

### Considerations:
- Models stored locally (~4-8GB per model)
- Requires sufficient RAM (8GB+ recommended)
- GPU optional but recommended for speed

## 📚 Resources

- **Ollama Website**: https://ollama.ai
- **Model Library**: https://ollama.ai/library
- **GitHub**: https://github.com/ollama/ollama
- **Discord**: https://discord.gg/ollama
- **Documentation**: https://github.com/ollama/ollama/blob/main/docs/api.md

## 🎯 Best Practices

1. **Start Simple**: Begin with `llama2` or `phi`
2. **Monitor Resources**: Check RAM/CPU usage
3. **Use GPU**: Enable GPU for 5-10x speed improvement
4. **Cache Responses**: Consider caching for repeated questions
5. **Fallback Always**: Keep fallback system active
6. **Test Locally**: Test new models before production
7. **Update Models**: Regularly update to latest versions

## 🚀 Quick Start

```bash
# 1. Install Ollama
curl https://ollama.ai/install.sh | sh

# 2. Pull model
ollama pull llama2

# 3. Start Ollama
ollama serve

# 4. Enable in .env
echo "OLLAMA_ENABLED=true" >> .env

# 5. Build & run
cargo build
cargo run

# 6. Test
curl -X POST http://localhost:3000/chat/send \
  -H "Authorization: Bearer rahasia_token_anda_yang_kuat_123456" \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello!"}'
```

---

**Status**: ✅ Production Ready  
**Version**: 3.0.0 - Ollama Integration  
**Last Updated**: November 2025
