use std::path::{Path, PathBuf};
use anyhow::Result;
use ffmpeg_next as ffmpeg;

use super::VideoSpec;

pub struct VideoGenerator {
    output_dir: PathBuf,
    temp_dir: PathBuf,
}

impl VideoGenerator {
    pub fn new(output_dir: impl Into<PathBuf>, temp_dir: impl Into<PathBuf>) -> Result<Self> {
        ffmpeg::init()?;
        
        let output_dir = output_dir.into();
        let temp_dir = temp_dir.into();
        
        std::fs::create_dir_all(&output_dir)?;
        std::fs::create_dir_all(&temp_dir)?;
        
        Ok(Self {
            output_dir,
            temp_dir,
        })
    }
    
    pub fn generate_video(&self, spec: &VideoSpec) -> Result<PathBuf> {
        let output_path = self.output_dir.join(format!("{}.mp4", 
            spec.title.chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .collect::<String>()
        ));
        
        match &spec.template {
            super::VideoTemplate::SimpleText => {
                self.generate_simple_text_video(spec, &output_path)?;
            }
            super::VideoTemplate::TitleCard => {
                todo!("TitleCard template not implemented yet");
            }
            super::VideoTemplate::Slideshow { slides: _ } => {
                todo!("Slideshow template not implemented yet");
            }
        }
        
        Ok(output_path)
    }
    
    fn generate_simple_text_video(&self, spec: &VideoSpec, output_path: &Path) -> Result<()> {
        // Create a solid color video with text overlay
        // Target: 1080x1920 (9:16 aspect ratio), 30fps
        
        let mut output = ffmpeg::format::output(&output_path)?;
        let global_header = output.format().flags().contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);
        
        // Add video stream
        let mut video_stream = output.add_stream(ffmpeg::encoder::find(ffmpeg::codec::Id::H264))?;
        let video_context = ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())?;
        let mut video_encoder = video_context.encoder().video()?;
        
        // Configure video encoder
        video_encoder.set_width(1080);
        video_encoder.set_height(1920);
        video_encoder.set_format(ffmpeg::format::Pixel::YUV420P);
        video_encoder.set_time_base((1, 30)); // 30fps
        video_encoder.set_frame_rate(Some((30, 1)));
        video_encoder.set_bit_rate(2_000_000); // 2 Mbps
        video_encoder.set_max_bit_rate(2_500_000); // 2.5 Mbps max
        video_encoder.set_gop(30); // GOP size
        video_encoder.set_qmin(10);
        video_encoder.set_qmax(51);
        
        if global_header {
            video_encoder.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
        }
        
        let video_encoder = video_encoder.open_as(ffmpeg::encoder::find(ffmpeg::codec::Id::H264))?;
        video_stream.set_parameters(&video_encoder);
        
        // Store time bases before borrowing output mutably
        let stream_time_base = video_stream.time_base();
        let encoder_time_base = video_encoder.time_base();
        
        // Write header
        output.write_header()?;
        
        // Generate frames
        let total_frames = spec.duration_seconds * 30; // 30fps
        let mut frame = ffmpeg::frame::Video::new(ffmpeg::format::Pixel::YUV420P, 1080, 1920);
        
        // Fill frame with solid color (black for now)
        // YUV420P format: Y=16 (black), U=128, V=128 (neutral chroma)
        unsafe {
            let y_size = 1080 * 1920;
            let uv_size = (1080 / 2) * (1920 / 2);
            
            // Y plane (luminance) - set to 16 for black
            std::ptr::write_bytes(frame.data_mut(0).as_mut_ptr(), 16, y_size);
            // U plane (chroma) - set to 128 for neutral
            std::ptr::write_bytes(frame.data_mut(1).as_mut_ptr(), 128, uv_size);
            // V plane (chroma) - set to 128 for neutral  
            std::ptr::write_bytes(frame.data_mut(2).as_mut_ptr(), 128, uv_size);
        }
        
        // Create a new encoder for the actual encoding process
        let mut encoder = video_encoder;
        
        for i in 0..total_frames {
            frame.set_pts(Some(i as i64));
            
            // Fill frame with background color (simplified - just black for now)
            // TODO: Parse background_color hex and convert to YUV
            // TODO: Add text overlay using spec.title, text_color, font_size
            
            encoder.send_frame(&frame)?;
            
            let mut encoded = ffmpeg::packet::Packet::empty();
            while encoder.receive_packet(&mut encoded).is_ok() {
                encoded.set_stream(0);
                encoded.rescale_ts(encoder_time_base, stream_time_base);
                encoded.write_interleaved(&mut output)?;
            }
        }
        
        // Flush encoder
        encoder.send_eof()?;
        let mut encoded = ffmpeg::packet::Packet::empty();
        while encoder.receive_packet(&mut encoded).is_ok() {
            encoded.set_stream(0);
            encoded.rescale_ts(encoder_time_base, stream_time_base);
            encoded.write_interleaved(&mut output)?;
        }
        
        output.write_trailer()?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_video_generator_creation() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().join("output");
        let temp_path = temp_dir.path().join("temp");
        
        let generator = VideoGenerator::new(&output_dir, &temp_path);
        assert!(generator.is_ok());
        
        assert!(output_dir.exists());
        assert!(temp_path.exists());
    }
    
    #[test]
    fn test_simple_text_video_generation() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().join("output");
        let temp_path = temp_dir.path().join("temp");
        
        let generator = VideoGenerator::new(&output_dir, &temp_path).unwrap();
        
        let spec = VideoSpec {
            title: "Test Video".to_string(),
            template: super::VideoTemplate::SimpleText,
            duration_seconds: 5,
            background_color: "#1a1a1a".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 48,
            audio_track: None,
        };
        
        let result = generator.generate_video(&spec);
        assert!(result.is_ok());
        
        let output_path = result.unwrap();
        assert!(output_path.exists());
        assert!(output_path.extension().unwrap() == "mp4");
    }
}
