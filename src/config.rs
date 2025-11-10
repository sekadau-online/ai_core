use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub bearer_token: String,
    pub api_host: String,
    pub api_port: u16,
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

        Ok(Self {
            bearer_token,
            api_host,
            api_port,
        })
    }

    /// Get the full API address
    pub fn address(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}
