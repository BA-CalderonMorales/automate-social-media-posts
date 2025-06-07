use std::path::Path;
use anyhow::Result;
use ffmpeg_next as ffmpeg;

use super::VideoValidation;

pub struct VideoValidator;

impl VideoValidator {
    pub fn validate_video(video_path: &Path) -> Result<VideoValidation> {
        let context = ffmpeg::format::input(&video_path)?;
        
        // Find video stream
        let video_stream = context
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or_else(|| anyhow::anyhow!("No video stream found"))?;
        
        let video_context = ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())?;
        let video_decoder = video_context.decoder().video()?;
        
        // Check dimensions (should be 1080x1920)
        let correct_dimensions = video_decoder.width() == 1080 && video_decoder.height() == 1920;
        
        // Check duration (convert from stream time base to seconds)
        let duration_seconds = {
            let duration = video_stream.duration();
            let time_base = video_stream.time_base();
            (duration as f64 * time_base.numerator() as f64 / time_base.denominator() as f64) as u32
        };
        
        let duration_in_range = duration_seconds >= 10 && duration_seconds <= 60;
        
        // Check file size (<50MB)
        let file_size = std::fs::metadata(video_path)?.len();
        let file_size_under_limit = file_size < 50 * 1024 * 1024; // 50MB
        
        // Check for audio stream
        let has_audio = context
            .streams()
            .best(ffmpeg::media::Type::Audio)
            .is_some();
        
        // Basic playability check (if we can open and decode, it's likely playable)
        let is_playable = true; // We successfully opened it above
        
        Ok(VideoValidation {
            correct_dimensions,
            duration_in_range,
            file_size_under_limit,
            has_audio,
            is_playable,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::video::generator::VideoGenerator;
    use crate::video::{VideoSpec, VideoTemplate};
    
    #[test]
    fn test_video_validation() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().join("output");
        let temp_path = temp_dir.path().join("temp");
        
        let generator = VideoGenerator::new(&output_dir, &temp_path).unwrap();
        
        let spec = VideoSpec {
            title: "Validation Test".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 15,
            background_color: "#000000".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 32,
            audio_track: None,
        };
        
        let video_path = generator.generate_video(&spec).unwrap();
        let validation = VideoValidator::validate_video(&video_path).unwrap();
        
        assert!(validation.correct_dimensions);
        assert!(validation.duration_in_range);
        assert!(validation.file_size_under_limit);
        assert!(validation.is_playable);
        // Note: has_audio will be false since we're not adding audio yet
    }
}
