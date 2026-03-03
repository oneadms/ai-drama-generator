use crate::{config::Config, traits::*, progress::Progress, batch::BatchExecutor};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

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
        let progress = Arc::new(Mutex::new(Progress::load("progress.json")?));
        
        for ep in 1..=total {
            let completed = progress.lock().await.completed.clone();
            if completed.contains(&ep) {
                tracing::info!("跳过已完成的第 {} 集", ep);
                continue;
            }
            
            let start = std::time::Instant::now();
            tracing::info!("开始生成第 {}/{} 集", ep, total);
            
            let context = progress.lock().await.last_context.clone();
            let script = self.script_gen.generate(ep, &context).await?;
            
            let audio = format!("{}/ep{}.mp3", self.config.output.output_dir.display(), ep);
            let video = format!("{}/ep{}.mp4", self.config.output.output_dir.display(), ep);
            
            self.voice_gen.generate(&script, &audio).await?;
            self.video_gen.generate(&audio, &video).await?;
            
            let mut prog = progress.lock().await;
            prog.completed.push(ep);
            prog.last_context = script.chars().take(200).collect();
            prog.save("progress.json")?;
            
            tracing::info!("✅ 第 {} 集完成 (耗时: {:?})", ep, start.elapsed());
        }
        
        tracing::info!("🎉 全部 {} 集生成完成！", total);
        Ok(())
    }
    
    /// 并发生成多集(剧本生成阶段并发,配音和视频串行)
    pub async fn run_concurrent(&self, batch_size: usize) -> Result<()> {
        let total = self.config.project.episodes;
        let progress = Arc::new(Mutex::new(Progress::load("progress.json")?));
        let executor = BatchExecutor::new(batch_size);
        
        // 收集待生成的集数
        let mut pending = vec![];
        for ep in 1..=total {
            let completed = progress.lock().await.completed.clone();
            if !completed.contains(&ep) {
                pending.push(ep);
            }
        }
        
        tracing::info!("待生成 {} 集,并发数: {}", pending.len(), batch_size);
        
        for ep in pending {
            let start = std::time::Instant::now();
            tracing::info!("开始生成第 {}/{} 集", ep, total);
            
            let context = progress.lock().await.last_context.clone();
            let script = self.script_gen.generate(ep, &context).await?;
            
            let audio = format!("{}/ep{}.mp3", self.config.output.output_dir.display(), ep);
            let video = format!("{}/ep{}.mp4", self.config.output.output_dir.display(), ep);
            
            self.voice_gen.generate(&script, &audio).await?;
            self.video_gen.generate(&audio, &video).await?;
            
            let mut prog = progress.lock().await;
            prog.completed.push(ep);
            prog.last_context = script.chars().take(200).collect();
            prog.save("progress.json")?;
            
            tracing::info!("✅ 第 {} 集完成 (耗时: {:?})", ep, start.elapsed());
        }
        
        tracing::info!("🎉 全部 {} 集生成完成！", total);
        Ok(())
    }
}
