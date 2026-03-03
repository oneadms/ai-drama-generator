use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    // API 相关
    #[error("API request failed: {0}")]
    ApiError(String),
    
    #[error("API rate limit exceeded, retry after {retry_after}s")]
    RateLimitExceeded { retry_after: u64 },
    
    #[error("API authentication failed: {0}")]
    AuthenticationError(String),
    
    // 网络相关
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Connection timeout after {0}s")]
    Timeout(u64),
    
    // 输入验证
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Missing parameter: {0}")]
    MissingParameter(String),
    
    // 文件操作
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    
    // 资源错误
    #[error("Resource not initialized")]
    NotInitialized,
    
    // 业务逻辑
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
}

impl GeneratorError {
    /// 判断是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::Timeout(_) | Self::RateLimitExceeded { .. }
        )
    }
    
    /// 获取重试延迟(秒)
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            Self::RateLimitExceeded { retry_after } => Some(*retry_after),
            Self::NetworkError(_) => Some(2),
            Self::Timeout(_) => Some(5),
            _ => None,
        }
    }
    
    /// 判断是否致命错误
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::AuthenticationError(_) | Self::InvalidInput(_))
    }
}

pub type Result<T> = std::result::Result<T, GeneratorError>;
