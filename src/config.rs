use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub script: ScriptConfig,
    pub voice: VoiceConfig,
    pub video: VideoConfig,
    pub assets: AssetsConfig,
    pub output: OutputConfig,
    pub upload: UploadConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub genre: String,
    pub episodes: u32,
    pub duration_per_episode: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScriptConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub temperature: f32,
    pub template_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoiceConfig {
    pub provider: String,
    pub voice_name: String,
    pub api_key: String,
    pub region: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoConfig {
    pub provider: String,
    pub avatar_image: PathBuf,
    pub resolution: String,
    pub fps: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetsConfig {
    pub background_music: PathBuf,
    pub sound_effects: PathBuf,
    pub background_images: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub output_dir: PathBuf,
    pub format: String,
    pub quality: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadConfig {
    pub enabled: bool,
    pub platform: String,
    pub schedule: String,
    pub episodes_per_day: u32,
}
