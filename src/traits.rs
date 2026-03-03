use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Episode {
    pub number: u32,
    pub script: String,
    pub audio_path: Option<String>,
    pub video_path: Option<String>,
}

#[async_trait]
pub trait ScriptGenerator: Send + Sync {
    async fn generate(&self, episode: u32, context: &str) -> Result<String>;
}

#[async_trait]
pub trait VoiceGenerator: Send + Sync {
    async fn generate(&self, text: &str, output: &str) -> Result<()>;
}

#[async_trait]
pub trait VideoGenerator: Send + Sync {
    async fn generate(&self, audio: &str, output: &str) -> Result<()>;
}

#[async_trait]
pub trait Uploader: Send + Sync {
    async fn upload(&self, video: &str, metadata: &EpisodeMetadata) -> Result<()>;
}

#[derive(Debug)]
pub struct EpisodeMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}
