mod config;
mod traits;
mod generators;
mod pipeline;
mod factory;
mod progress;
mod batch;
mod tempfile;
mod errors;

use anyhow::Result;
use config::Config;
use pipeline::Pipeline;
use factory::*;
use std::fs;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    
    fs::create_dir_all(&config.output.output_dir)?;
    
    let script_gen = create_script_generator(&config.script)?;
    let voice_gen = create_voice_generator(&config.voice)?;
    let video_gen = create_video_generator(&config.video)?;
    
    let pipeline = Pipeline::new(config, script_gen, voice_gen, video_gen);
    pipeline.run().await?;
    
    Ok(())
}
