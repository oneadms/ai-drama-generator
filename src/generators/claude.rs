use crate::traits::ScriptGenerator;
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::fs;

pub struct ClaudeGenerator {
    api_key: String,
    model: String,
    template: String,
    client: Client,
}

impl ClaudeGenerator {
    pub fn new(api_key: String, model: String, template_path: &str) -> Result<Self> {
        let template = fs::read_to_string(template_path).unwrap_or_default();
        Ok(Self { api_key, model, template, client: Client::new() })
    }
}

#[async_trait]
impl ScriptGenerator for ClaudeGenerator {
    async fn generate(&self, episode: u32, context: &str) -> Result<String> {
        let prompt = self.template
            .replace("{episode}", &episode.to_string())
            .replace("{context}", context);
        
        let res = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": self.model,
                "max_tokens": 2000,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await?;
        
        let data: serde_json::Value = res.json().await?;
        Ok(data["content"][0]["text"].as_str().unwrap().to_string())
    }
}
