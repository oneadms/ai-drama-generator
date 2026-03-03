use crate::traits::VoiceGenerator;
use async_trait::async_trait;
use anyhow::Result;
use tokio::process::Command;

pub struct EdgeTTS {
    voice: String,
}

impl EdgeTTS {
    pub fn new(voice: String) -> Self {
        Self { voice }
    }
}

#[async_trait]
impl VoiceGenerator for EdgeTTS {
    async fn generate(&self, text: &str, output: &str) -> Result<()> {
        tracing::info!("Edge TTS 生成: {}", output);
        
        let status = Command::new("edge-tts")
            .arg("--voice").arg(&self.voice)
            .arg("--text").arg(text)
            .arg("--write-media").arg(output)
            .status()
            .await?;
        
        if !status.success() {
            anyhow::bail!("Edge TTS 失败");
        }
        
        Ok(())
    }
}
