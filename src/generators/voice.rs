use crate::traits::VoiceGenerator;
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use std::fs;

pub struct AzureTTS {
    api_key: String,
    region: String,
    voice: String,
    client: Client,
}

impl AzureTTS {
    pub fn new(api_key: String, region: String, voice: String) -> Self {
        Self { api_key, region, voice, client: Client::new() }
    }
}

#[async_trait]
impl VoiceGenerator for AzureTTS {
    async fn generate(&self, text: &str, output: &str) -> Result<()> {
        tracing::info!("生成配音: {}", output);
        
        let ssml = format!(
            r#"<speak version='1.0' xml:lang='zh-CN'>
                <voice name='{}'>{}</voice>
            </speak>"#,
            self.voice, text
        );
        
        let url = format!("https://{}.tts.speech.microsoft.com/cognitiveservices/v1", self.region);
        
        let res = self.client
            .post(&url)
            .header("Ocp-Apim-Subscription-Key", &self.api_key)
            .header("Content-Type", "application/ssml+xml")
            .header("X-Microsoft-OutputFormat", "audio-16khz-128kbitrate-mono-mp3")
            .body(ssml)
            .send()
            .await?;
        
        let audio = res.bytes().await?;
        fs::write(output, audio)?;
        
        Ok(())
    }
}
