use crate::traits::ScriptGenerator;
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::fs;

pub struct OpenAIGenerator {
    api_key: String,
    model: String,
    template: String,
    client: Client,
}

impl OpenAIGenerator {
    pub fn new(api_key: String, model: String, template_path: &str) -> Result<Self> {
        let template = fs::read_to_string(template_path).unwrap_or_default();
        Ok(Self { api_key, model, template, client: Client::new() })
    }
}

#[async_trait]
impl ScriptGenerator for OpenAIGenerator {
    async fn generate(&self, episode: u32, context: &str) -> Result<String> {
        let prompt = self.template
            .replace("{episode}", &episode.to_string())
            .replace("{context}", context);
        
        let res = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await?;
        
        let data: serde_json::Value = res.json().await?;
        Ok(data["choices"][0]["message"]["content"].as_str().unwrap().to_string())
    }
}
