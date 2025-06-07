use async_trait::async_trait;
use std::path::Path;
use thiserror::Error;

#[derive(Debug)]
pub struct VideoMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub privacy: PrivacyLevel,
}

#[derive(Debug)]
pub enum PrivacyLevel {
    Public,
    Private,
    Unlisted,
}

#[derive(Debug)]
pub struct UploadResult {
    pub video_id: String,
    pub platform: String,
    pub upload_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("File error: {0}")]
    File(#[from] std::io::Error),
}

#[async_trait]
pub trait VideoPlatform {
    async fn upload_video(
        &self,
        video_path: &Path,
        metadata: VideoMetadata,
    ) -> Result<UploadResult, PlatformError>;
    
    fn platform_name(&self) -> &str;
    fn max_file_size(&self) -> u64;
    fn supported_formats(&self) -> Vec<String>;
}

// TODO: Implement real platform integrations and mock platform for testing in later phases
