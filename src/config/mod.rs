use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub video: VideoConfig,
    pub scheduling: SchedulingConfig,
    pub platforms: PlatformsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoConfig {
    pub output_directory: String,
    pub temp_directory: String,
    pub cleanup_after_upload: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchedulingConfig {
    pub timezone: String,
    pub daily_post_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformsConfig {
    pub youtube: PlatformConfig,
    pub tiktok: PlatformConfig,
    pub mock: PlatformConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub enabled: bool,
    pub upload_as_private: Option<bool>,
    pub max_daily_uploads: Option<u32>,
    pub simulate_failures: Option<bool>,
}

// TODO: Implement config loading and validation in later phases
