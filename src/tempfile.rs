use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

/// 临时文件管理器,自动清理失败的临时文件
pub struct TempFileManager {
    temp_files: Vec<PathBuf>,
}

impl TempFileManager {
    pub fn new() -> Self {
        Self { temp_files: vec![] }
    }
    
    /// 注册临时文件
    pub fn register<P: AsRef<Path>>(&mut self, path: P) {
        self.temp_files.push(path.as_ref().to_path_buf());
    }
    
    /// 确认文件成功,不再清理
    pub fn confirm<P: AsRef<Path>>(&mut self, path: P) {
        self.temp_files.retain(|p| p != path.as_ref());
    }
    
    /// 清理所有临时文件
    pub fn cleanup(&mut self) -> Result<()> {
        for path in &self.temp_files {
            if path.exists() {
                fs::remove_file(path)?;
                tracing::info!("清理临时文件: {}", path.display());
            }
        }
        self.temp_files.clear();
        Ok(())
    }
}

impl Drop for TempFileManager {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
