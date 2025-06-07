use anyhow::Result;
use automate_social_media_posts::video::{generator::VideoGenerator, validation::VideoValidator, VideoSpec, VideoTemplate};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Video Generation Test - Task 1.1.1: FFmpeg wrapper setup");
    
    let generator = VideoGenerator::new("output", "temp")?;
    
    let test_spec = VideoSpec {
        title: "Test Video Generation".to_string(),
        template: VideoTemplate::SimpleText,
        duration_seconds: 10,
        background_color: "#1a1a1a".to_string(),
        text_color: "#ffffff".to_string(),
        font_size: 48,
        audio_track: None,
    };
    
    println!("Generating test video...");
    let output_path = generator.generate_video(&test_spec)?;
    println!("Video generated: {}", output_path.display());
    
    // Validate the generated video
    let validation = VideoValidator::validate_video(&output_path)?;
    println!("Validation results:");
    println!("  Correct dimensions (1080x1920): {}", validation.correct_dimensions);
    println!("  Duration in range (10-60s): {}", validation.duration_in_range);
    println!("  File size under limit (<50MB): {}", validation.file_size_under_limit);
    println!("  Has audio: {}", validation.has_audio);
    println!("  Is playable: {}", validation.is_playable);
    println!("  Overall valid: {}", validation.is_valid());
    
    Ok(())
}
