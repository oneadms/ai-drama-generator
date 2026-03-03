use crate::traits::ScriptGenerator;
use async_trait::async_trait;
use anyhow::{Result, Context, bail};
use reqwest::Client;
use serde_json::json;
use std::fs;
use tokio::time::{sleep, Duration};

pub struct OpenAIGenerator {
    api_key: String,
    model: String,
    template: String,
    client: Client,
}

impl OpenAIGenerator {
    pub fn new(api_key: String, model: String, template_path: &str) -> Result<Self> {
        let template = fs::read_to_string(template_path)
            .context("Failed to read template file")?;
        Ok(Self { api_key, model, template, client: Client::new() })
    }
    
    async fn generate_with_retry(&self, episode: u32, context: &str) -> Result<String> {
        let mut retries = 0;
        let max_retries = 3;
        
        loop {
            match self.generate_once(episode, context).await {
                Ok(result) => return Ok(result),
                Err(e) if retries < max_retries => {
                    retries += 1;
                    let delay = Duration::from_secs(2u64.pow(retries));
                    tracing::warn!("API 调用失败 (重试 {}/{}): {}", retries, max_retries, e);
                    sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
    
    async fn generate_once(&self, episode: u32, context: &str) -> Result<String> {
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
            .await
            .context("Failed to send request to OpenAI API")?;
        
        if !res.status().is_success() {
            bail!("OpenAI API returned error: {}", res.status());
        }
        
        let data: serde_json::Value = res.json().await
            .context("Failed to parse OpenAI API response")?;
        
        data["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .context("Invalid response format from OpenAI API")
    }
}

#[async_trait]
impl ScriptGenerator for OpenAIGenerator {
    async fn generate(&self, episode: u32, context: &str) -> Result<String> {
        self.generate_with_retry(episode, context).await
    }
}
