use crate::traits::VideoGenerator;
use async_trait::async_trait;
use anyhow::Result;
use tokio::process::Command;

pub struct SadTalkerGenerator {
    avatar_path: String,
}

impl SadTalkerGenerator {
    pub fn new(avatar_path: String) -> Self {
        Self { avatar_path }
    }
}

#[async_trait]
impl VideoGenerator for SadTalkerGenerator {
    async fn generate(&self, audio: &str, output: &str) -> Result<()> {
        tracing::info!("生成视频: {} -> {}", audio, output);
        
        let status = Command::new("python3")
            .arg("scripts/sadtalker.py")
            .arg("--driven_audio").arg(audio)
            .arg("--source_image").arg(&self.avatar_path)
            .arg("--result_dir").arg(output)
            .status()
            .await?;
        
        if !status.success() {
            anyhow::bail!("SadTalker 生成失败");
        }
        
        Ok(())
    }
}
