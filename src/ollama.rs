use serde::{Deserialize, Serialize};

/// Ollama client for AI response generation
pub struct OllamaClient {
    url: String,
    model: String,
    enabled: bool,
}

/// Ollama request structure
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

/// Ollama response structure
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    #[serde(default)]
    response: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    error: Option<String>,
}

impl OllamaClient {
    /// Create new Ollama client from config
    pub fn new(url: String, model: String, enabled: bool) -> Self {
        Self {
            url,
            model,
            enabled,
        }
    }

    /// Check if Ollama is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Generate AI response using Ollama
    pub async fn generate(&self, prompt: &str) -> Result<String, String> {
        if !self.enabled {
            return Err("Ollama is disabled. Set OLLAMA_ENABLED=true in .env".to_string());
        }

        let client = reqwest::Client::new();
        let endpoint = format!("{}/api/generate", self.url);

        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        tracing::debug!("Sending request to Ollama: {}", endpoint);
        tracing::debug!("Model: {}", self.model);
        tracing::debug!("Prompt length: {} chars", prompt.len());

        match client.post(&endpoint)
            .json(&request_body)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                tracing::debug!("Ollama response status: {}", status);

                if !status.is_success() {
                    let error_text = response.text().await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    return Err(format!("Ollama API error ({}): {}", status, error_text));
                }

                match response.json::<OllamaResponse>().await {
                    Ok(ollama_response) => {
                        if let Some(error) = ollama_response.error {
                            return Err(format!("Ollama error: {}", error));
                        }

                        if ollama_response.response.is_empty() {
                            return Err("Ollama returned empty response".to_string());
                        }

                        tracing::debug!("Ollama response length: {} chars", ollama_response.response.len());
                        Ok(ollama_response.response)
                    }
                    Err(e) => {
                        Err(format!("Failed to parse Ollama response: {}", e))
                    }
                }
            }
            Err(e) => {
                Err(format!("Failed to connect to Ollama: {}. Make sure Ollama is running at {}", e, self.url))
            }
        }
    }

    /// Generate response with context from memory
    pub async fn generate_with_context(
        &self,
        user_input: &str,
        context: &[String],
    ) -> Result<String, String> {
        let context_text = if context.is_empty() {
            String::from("No context available.")
        } else {
            format!("Context from memory:\n{}", context.join("\n"))
        };

        let prompt = format!(
            "{}\n\nUser question: {}\n\nPlease provide a helpful response based on the context above.",
            context_text,
            user_input
        );

        self.generate(&prompt).await
    }

    /// Check if Ollama server is available
    pub async fn health_check(&self) -> bool {
        if !self.enabled {
            return false;
        }

        let client = reqwest::Client::new();
        let endpoint = format!("{}/api/tags", self.url);

        match client.get(&endpoint)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let is_ok = response.status().is_success();
                tracing::info!("Ollama health check: {}", if is_ok { "OK" } else { "FAILED" });
                is_ok
            }
            Err(e) => {
                tracing::warn!("Ollama health check failed: {}", e);
                false
            }
        }
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>, String> {
        if !self.enabled {
            return Err("Ollama is disabled".to_string());
        }

        let client = reqwest::Client::new();
        let endpoint = format!("{}/api/tags", self.url);

        match client.get(&endpoint)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) => {
                #[derive(Deserialize)]
                struct TagsResponse {
                    models: Vec<ModelInfo>,
                }

                #[derive(Deserialize)]
                struct ModelInfo {
                    name: String,
                }

                match response.json::<TagsResponse>().await {
                    Ok(tags) => {
                        let model_names: Vec<String> = tags.models
                            .into_iter()
                            .map(|m| m.name)
                            .collect();
                        Ok(model_names)
                    }
                    Err(e) => Err(format!("Failed to parse models list: {}", e))
                }
            }
            Err(e) => {
                Err(format!("Failed to fetch models: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ollama_client_disabled() {
        let client = OllamaClient::new(
            "http://localhost:11434".to_string(),
            "llama2".to_string(),
            false,
        );

        assert!(!client.is_enabled());
        
        let result = client.generate("test").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("disabled"));
    }
}
