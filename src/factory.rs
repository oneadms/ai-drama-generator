use crate::{config::*, generators::*, traits::*};
use anyhow::{Result, bail};
use std::sync::Arc;

pub fn create_script_generator(cfg: &ScriptConfig) -> Result<Arc<dyn ScriptGenerator>> {
    let template = cfg.template_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid template path"))?;
    match cfg.provider.as_str() {
        "openai" => Ok(Arc::new(OpenAIGenerator::new(cfg.api_key.clone(), cfg.model.clone(), template)?)),
        "claude" => Ok(Arc::new(ClaudeGenerator::new(cfg.api_key.clone(), cfg.model.clone(), template)?)),
        _ => bail!("未知的脚本生成器: {}", cfg.provider),
    }
}

pub fn create_voice_generator(cfg: &VoiceConfig) -> Result<Arc<dyn VoiceGenerator>> {
    match cfg.provider.as_str() {
        "azure" => Ok(Arc::new(AzureTTS::new(
            cfg.api_key.clone(),
            cfg.region.clone().unwrap_or_default(),
            cfg.voice_name.clone(),
        ))),
        "edge" => Ok(Arc::new(EdgeTTS::new(cfg.voice_name.clone()))),
        _ => bail!("未知的配音生成器: {}", cfg.provider),
    }
}

pub fn create_video_generator(cfg: &VideoConfig) -> Result<Arc<dyn VideoGenerator>> {
    match cfg.provider.as_str() {
        "sadtalker" => Ok(Arc::new(SadTalkerGenerator::new(
            cfg.avatar_image.display().to_string(),
        ))),
        _ => bail!("未知的视频生成器: {}", cfg.provider),
    }
}
