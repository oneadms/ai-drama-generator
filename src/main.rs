mod config;
mod traits;
mod generators;
mod pipeline;
mod factory;
mod progress;
mod batch;
mod tempfile;
mod errors;
mod api;

use anyhow::Result;
use config::Config;
use pipeline::Pipeline;
use factory::*;
use std::{fs, env};
use tracing_subscriber;
use axum::Router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let mode = env::args().nth(1).unwrap_or_else(|| "cli".to_string());
    
    if mode == "server" {
        run_server().await?;
    } else {
        run_cli().await?;
    }
    
    Ok(())
}

async fn run_server() -> Result<()> {
    let task_mgr = std::sync::Arc::new(api::TaskManager::new());
    
    let app = Router::new()
        .nest("/api", api::create_router(task_mgr))
        .layer(CorsLayer::permissive());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Server running on http://localhost:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_cli() -> Result<()> {
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
