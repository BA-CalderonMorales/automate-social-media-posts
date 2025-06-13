pub mod generator;
pub mod validation;

use serde::{Deserialize, Serialize};

// Re-export commonly used types
pub use generator::VideoGenerator;
pub use validation::VideoValidator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSpec {
    pub title: String,
    pub template: VideoTemplate,
    pub duration_seconds: u32,
    pub background_color: String, // hex color
    pub text_color: String,       // hex color
    pub font_size: u32,
    pub audio_track: Option<String>, // path to audio file
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoTemplate {
    SimpleText,
    TitleCard,
    Slideshow { slides: Vec<String> },
}

#[derive(Debug)]
pub struct VideoValidation {
    pub correct_dimensions: bool,
    pub duration_in_range: bool,
    pub file_size_under_limit: bool,
    pub has_audio: bool,
    pub is_playable: bool,
}

impl VideoValidation {
    pub fn is_valid(&self) -> bool {
        self.correct_dimensions
            && self.duration_in_range
            && self.file_size_under_limit
            && self.is_playable
    }
    
    /// Check if video meets production requirements (stricter duration)
    pub fn is_production_ready(&self, duration_seconds: u32) -> bool {
        self.is_valid() && duration_seconds >= 10 && duration_seconds <= 60
    }
    
    /// Get validation summary for logging
    pub fn get_summary(&self) -> String {
        format!(
            "Dimensions: {}, Duration: {}, Size: {}, Audio: {}, Playable: {}",
            if self.correct_dimensions { "✅" } else { "❌" },
            if self.duration_in_range { "✅" } else { "❌" },
            if self.file_size_under_limit { "✅" } else { "❌" },
            if self.has_audio { "✅" } else { "➖" },
            if self.is_playable { "✅" } else { "❌" }
        )
    }
}
