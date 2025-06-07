pub mod video;
pub mod content;
pub mod platforms;
pub mod config;

pub use video::{VideoSpec, VideoTemplate, VideoValidation};
pub use content::{ContentItem, ContentSelector};
pub use platforms::{VideoPlatform, VideoMetadata, UploadResult, PlatformError};
pub use config::Config;
