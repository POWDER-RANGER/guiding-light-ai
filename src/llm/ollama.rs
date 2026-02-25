use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
}

pub async fn reflect(base_url: &str, model: &str, text: &str) -> Result<String> {
    let client = Client::new();
    let url = format!("{}{}" , base_url.trim_end_matches('/'), "/api/generate");
    
    let body = OllamaRequest {
        model: model.to_string(),
        prompt: format!("Consider this reflection: {}\n\nRespond with brief, thoughtful guidance.", text),
        stream: false,
    };
    
    let resp = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow!("Failed calling Ollama: {}", e))?;
    
    if !resp.status().is_success() {
        return Err(anyhow!("Ollama API error: {}", resp.status()));
    }
    
    let ollama_resp: OllamaResponse = resp
        .json()
        .await
        .map_err(|e| anyhow!("Failed parsing Ollama response: {}", e))?;
    
    Ok(ollama_resp.response)
}
