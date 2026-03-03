use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("API 调用失败: {0}")]
    ApiError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
    
    #[error("认证失败: {0}")]
    AuthError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("文件操作失败: {0}")]
    FileError(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("超时: {0}")]
    TimeoutError(String),
    
    #[error("资源不足: {0}")]
    ResourceError(String),
}

impl GeneratorError {
    /// 判断错误是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::TimeoutError(_) | Self::ApiError(_)
        )
    }
    
    /// 判断是否致命错误
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::AuthError(_) | Self::ConfigError(_))
    }
}
