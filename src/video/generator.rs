use ab_glyph::{FontRef, PxScale};
use anyhow::Result;
use ffmpeg_next as ffmpeg;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use std::path::{Path, PathBuf};

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

    /// Parse hex color string to RGB values
    fn parse_hex_color(hex: &str) -> Result<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(anyhow::anyhow!("Invalid hex color format: {}", hex));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok((r, g, b))
    }

    /// Convert RGB to YUV color space
    fn rgb_to_yuv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let y = (0.299 * r + 0.587 * g + 0.114 * b).max(0.0).min(1.0);
        let u = (-0.169 * r - 0.331 * g + 0.5 * b + 0.5).max(0.0).min(1.0);
        let v = (0.5 * r - 0.419 * g - 0.081 * b + 0.5).max(0.0).min(1.0);

        ((y * 255.0) as u8, (u * 255.0) as u8, (v * 255.0) as u8)
    }

    /// Create a text overlay image
    fn create_text_overlay(&self, spec: &VideoSpec) -> Result<RgbImage> {
        let width = 1080u32;
        let height = 1920u32;

        // Parse background color
        let (bg_r, bg_g, bg_b) = Self::parse_hex_color(&spec.background_color)?;
        let background_color = Rgb([bg_r, bg_g, bg_b]);

        // Parse text color
        let (text_r, text_g, text_b) = Self::parse_hex_color(&spec.text_color)?;
        let text_color = Rgb([text_r, text_g, text_b]);

        // Create image with background color
        let mut image = ImageBuffer::from_pixel(width, height, background_color);

        // Load default font (we'll use a simple built-in font for now)
        // In production, you'd load a proper TTF font file
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");
        let font = FontRef::try_from_slice(font_data)
            .map_err(|_| anyhow::anyhow!("Failed to load font"))?;

        let scale = PxScale::from(spec.font_size as f32);

        // Calculate text positioning (center horizontally, vertically centered)
        // For simplicity, we'll estimate text width and center it
        let estimated_char_width = spec.font_size as f32 * 0.6; // Rough estimation
        let text_width = spec.title.len() as f32 * estimated_char_width;

        let x = ((width as f32 - text_width) / 2.0).max(50.0) as i32;
        let y = (height as f32 / 2.0 - spec.font_size as f32 / 2.0) as i32;

        // Draw text on image
        draw_text_mut(&mut image, text_color, x, y, scale, &font, &spec.title);

        Ok(image)
    }

    pub fn generate_video(&self, spec: &VideoSpec) -> Result<PathBuf> {
        let output_path = self.output_dir.join(format!(
            "{}.mp4",
            spec.title
                .chars()
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
        // Create text overlay image first
        let overlay_image = self.create_text_overlay(spec)?;

        // Create a solid color video with text overlay
        // Target: 1080x1920 (9:16 aspect ratio), 30fps

        let mut output = ffmpeg::format::output(&output_path)?;
        let global_header = output
            .format()
            .flags()
            .contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);

        // Add video stream
        let mut video_stream = output.add_stream(ffmpeg::encoder::find(ffmpeg::codec::Id::H264))?;
        let video_context =
            ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())?;
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

        let video_encoder =
            video_encoder.open_as(ffmpeg::encoder::find(ffmpeg::codec::Id::H264))?;
        video_stream.set_parameters(&video_encoder);

        // Store time bases before borrowing output mutably
        let stream_time_base = video_stream.time_base();
        let encoder_time_base = video_encoder.time_base();

        // Write header
        output.write_header()?;

        // Generate frames
        let total_frames = spec.duration_seconds * 30; // 30fps
        let mut frame = ffmpeg::frame::Video::new(ffmpeg::format::Pixel::YUV420P, 1080, 1920);

        // Convert RGB image to YUV frame data
        self.fill_frame_with_image(&mut frame, &overlay_image)?;

        // Create a new encoder for the actual encoding process
        let mut encoder = video_encoder;

        for i in 0..total_frames {
            frame.set_pts(Some(i as i64));

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

    /// Fill FFmpeg frame with RGB image data converted to YUV
    fn fill_frame_with_image(
        &self,
        frame: &mut ffmpeg::frame::Video,
        image: &RgbImage,
    ) -> Result<()> {
        let width = image.width() as usize;
        let height = image.height() as usize;

        unsafe {
            let y_size = width * height;
            let uv_size = (width / 2) * (height / 2);

            let y_plane = std::slice::from_raw_parts_mut(frame.data_mut(0).as_mut_ptr(), y_size);
            let u_plane = std::slice::from_raw_parts_mut(frame.data_mut(1).as_mut_ptr(), uv_size);
            let v_plane = std::slice::from_raw_parts_mut(frame.data_mut(2).as_mut_ptr(), uv_size);

            // Convert RGB to YUV420P
            for y in 0..height {
                for x in 0..width {
                    let pixel = image.get_pixel(x as u32, y as u32);
                    let (yuv_y, yuv_u, yuv_v) = Self::rgb_to_yuv(pixel[0], pixel[1], pixel[2]);

                    // Y plane (full resolution)
                    y_plane[y * width + x] = yuv_y;

                    // U and V planes (half resolution - 4:2:0 subsampling)
                    if x % 2 == 0 && y % 2 == 0 {
                        let uv_idx = (y / 2) * (width / 2) + (x / 2);
                        u_plane[uv_idx] = yuv_u;
                        v_plane[uv_idx] = yuv_v;
                    }
                }
            }
        }

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
            template: crate::video::VideoTemplate::SimpleText,
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
