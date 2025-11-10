use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub bearer_token: String,
    pub api_host: String,
    pub api_port: u16,
    pub ollama_url: String,
    pub ollama_model: String,
    pub ollama_enabled: bool,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        let bearer_token = env::var("BEARER_TOKEN")
            .map_err(|_| "BEARER_TOKEN not set in .env file".to_string())?;

        let api_host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let api_port = env::var("API_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "Invalid API_PORT value".to_string())?;

        let ollama_url = env::var("OLLAMA_URL")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());

        let ollama_model = env::var("OLLAMA_MODEL")
            .unwrap_or_else(|_| "llama2".to_string());

        let ollama_enabled = env::var("OLLAMA_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        Ok(Self {
            bearer_token,
            api_host,
            api_port,
            ollama_url,
            ollama_model,
            ollama_enabled,
        })
    }

    /// Get the full API address
    pub fn address(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }

    /// Get Ollama API endpoint
    pub fn ollama_api_endpoint(&self) -> String {
        format!("{}/api/generate", self.ollama_url)
    }
}
