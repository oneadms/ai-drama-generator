use crate::{config::Config, traits::*, progress::Progress};
use anyhow::Result;
use std::sync::Arc;

pub struct Pipeline {
    config: Config,
    script_gen: Arc<dyn ScriptGenerator>,
    voice_gen: Arc<dyn VoiceGenerator>,
    video_gen: Arc<dyn VideoGenerator>,
}

impl Pipeline {
    pub fn new(
        config: Config,
        script_gen: Arc<dyn ScriptGenerator>,
        voice_gen: Arc<dyn VoiceGenerator>,
        video_gen: Arc<dyn VideoGenerator>,
    ) -> Self {
        Self { config, script_gen, voice_gen, video_gen }
    }

    pub async fn run(&self) -> Result<()> {
        let total = self.config.project.episodes;
        let mut progress = Progress::load("progress.json")?;
        
        for ep in 1..=total {
            if progress.completed.contains(&ep) {
                tracing::info!("跳过已完成的第 {} 集", ep);
                continue;
            }
            
            tracing::info!("生成第 {}/{} 集", ep, total);
            
            let script = self.script_gen.generate(ep, &progress.last_context).await?;
            let audio = format!("{}/ep{}.mp3", self.config.output.output_dir.display(), ep);
            let video = format!("{}/ep{}.mp4", self.config.output.output_dir.display(), ep);
            
            self.voice_gen.generate(&script, &audio).await?;
            self.video_gen.generate(&audio, &video).await?;
            
            progress.completed.push(ep);
            progress.last_context = script.chars().take(200).collect();
            progress.save("progress.json")?;
        }
        
        tracing::info!("✅ 全部完成！");
        Ok(())
    }
}
