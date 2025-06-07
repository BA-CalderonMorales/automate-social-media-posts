pub mod config;
pub mod content;
pub mod platforms;
pub mod video;

pub use config::Config;
pub use content::{ContentItem, ContentSelector};
pub use platforms::{PlatformError, UploadResult, VideoMetadata, VideoPlatform};
pub use video::{VideoSpec, VideoTemplate, VideoValidation};
