pub mod generator;
pub mod validation;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSpec {
    pub title: String,
    pub template: VideoTemplate,
    pub duration_seconds: u32,
    pub background_color: String,    // hex color
    pub text_color: String,          // hex color  
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
}
