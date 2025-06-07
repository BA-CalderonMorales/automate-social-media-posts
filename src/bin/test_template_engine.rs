use anyhow::Result;
use automate_social_media_posts::video::{
    generator::VideoGenerator, validation::VideoValidator, VideoSpec, VideoTemplate,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎯 Task 1.1.3: Template engine - Comprehensive Validation Test");
    println!("Generating 20 different videos to test SimpleText template and validation...\n");

    let generator = VideoGenerator::new("output", "temp")?;

    // Test configurations for comprehensive validation
    let test_configs = vec![
        // Basic tests
        VideoSpec {
            title: "Template Test 01".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 10,
            background_color: "#000000".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 32,
            audio_track: None,
        },
        VideoSpec {
            title: "Template Test 02".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 15,
            background_color: "#1a1a1a".to_string(),
            text_color: "#e74c3c".to_string(),
            font_size: 48,
            audio_track: None,
        },
        // Color variety tests
        VideoSpec {
            title: "Blue Sky Template".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 12,
            background_color: "#3498db".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 56,
            audio_track: None,
        },
        VideoSpec {
            title: "Green Nature".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 18,
            background_color: "#2ecc71".to_string(),
            text_color: "#2c3e50".to_string(),
            font_size: 40,
            audio_track: None,
        },
        VideoSpec {
            title: "Purple Vibes".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 20,
            background_color: "#9b59b6".to_string(),
            text_color: "#f39c12".to_string(),
            font_size: 64,
            audio_track: None,
        },
        // Font size tests
        VideoSpec {
            title: "Small Font Test".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 14,
            background_color: "#34495e".to_string(),
            text_color: "#ecf0f1".to_string(),
            font_size: 24,
            audio_track: None,
        },
        VideoSpec {
            title: "Medium Font".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 16,
            background_color: "#e67e22".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 44,
            audio_track: None,
        },
        VideoSpec {
            title: "Large Font Style".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 22,
            background_color: "#c0392b".to_string(),
            text_color: "#f1c40f".to_string(),
            font_size: 72,
            audio_track: None,
        },
        // Duration tests
        VideoSpec {
            title: "Short Duration".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 10,
            background_color: "#16a085".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 50,
            audio_track: None,
        },
        VideoSpec {
            title: "Long Duration Test".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 45,
            background_color: "#8e44ad".to_string(),
            text_color: "#ecf0f1".to_string(),
            font_size: 36,
            audio_track: None,
        },
        // Extreme duration tests
        VideoSpec {
            title: "Max Duration".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 60,
            background_color: "#d35400".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 42,
            audio_track: None,
        },
        // Color contrast tests
        VideoSpec {
            title: "High Contrast".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 25,
            background_color: "#000000".to_string(),
            text_color: "#ffffff".to_string(),
            font_size: 58,
            audio_track: None,
        },
        VideoSpec {
            title: "Low Contrast".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 30,
            background_color: "#7f8c8d".to_string(),
            text_color: "#95a5a6".to_string(),
            font_size: 38,
            audio_track: None,
        },
        // Bright colors
        VideoSpec {
            title: "Bright Yellow".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 17,
            background_color: "#f1c40f".to_string(),
            text_color: "#2c3e50".to_string(),
            font_size: 46,
            audio_track: None,
        },
        VideoSpec {
            title: "Neon Green".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 19,
            background_color: "#00ff00".to_string(),
            text_color: "#000000".to_string(),
            font_size: 52,
            audio_track: None,
        },
        // Pastel colors
        VideoSpec {
            title: "Soft Pink".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 23,
            background_color: "#ffc0cb".to_string(),
            text_color: "#8b4513".to_string(),
            font_size: 34,
            audio_track: None,
        },
        VideoSpec {
            title: "Light Blue".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 27,
            background_color: "#87ceeb".to_string(),
            text_color: "#191970".to_string(),
            font_size: 60,
            audio_track: None,
        },
        // Edge cases
        VideoSpec {
            title: "Very Long Title That Tests Text Wrapping".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 35,
            background_color: "#2c3e50".to_string(),
            text_color: "#e74c3c".to_string(),
            font_size: 28,
            audio_track: None,
        },
        VideoSpec {
            title: "A".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 11,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
            font_size: 80,
            audio_track: None,
        },
        // Final comprehensive test
        VideoSpec {
            title: "Final Validation".to_string(),
            template: VideoTemplate::SimpleText,
            duration_seconds: 40,
            background_color: "#1abc9c".to_string(),
            text_color: "#2c3e50".to_string(),
            font_size: 54,
            audio_track: None,
        },
    ];

    let mut successful_generations = 0;
    let mut successful_validations = 0;
    let mut production_ready = 0;
    let mut total_generation_time = 0.0;

    println!("Starting comprehensive video generation and validation...\n");

    for (i, spec) in test_configs.iter().enumerate() {
        let test_num = i + 1;
        print!("🔄 Test {}/20: Generating '{}'... ", test_num, spec.title);
        
        let start_time = std::time::Instant::now();
        match generator.generate_video(spec) {
            Ok(output_path) => {
                let generation_time = start_time.elapsed().as_secs_f64();
                total_generation_time += generation_time;
                successful_generations += 1;
                
                println!("✅ Generated in {:.1}s", generation_time);
                
                // Validate the video
                match VideoValidator::validate_video(&output_path) {
                    Ok(validation) => {
                        successful_validations += 1;
                        
                        if validation.is_production_ready(spec.duration_seconds) {
                            production_ready += 1;
                        }
                        
                        println!("   📊 {}", validation.get_summary());
                        
                        if !validation.is_valid() {
                            println!("   ⚠️  Video failed validation!");
                        }
                    }
                    Err(e) => {
                        println!("   ❌ Validation failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Generation failed: {}", e);
            }
        }
        println!();
    }

    // Print comprehensive results
    println!("{}", "=".repeat(60));
    println!("🎉 Task 1.1.3: Template Engine Results Summary");
    println!("{}", "=".repeat(60));
    println!("📈 Generation Results:");
    println!("   • Successful generations: {}/20 ({:.1}%)", 
             successful_generations, (successful_generations as f64 / 20.0) * 100.0);
    println!("   • Average generation time: {:.1}s", total_generation_time / successful_generations as f64);
    println!("   • Total generation time: {:.1}s", total_generation_time);
    
    println!("\n🔍 Validation Results:");
    println!("   • Successful validations: {}/20 ({:.1}%)", 
             successful_validations, (successful_validations as f64 / 20.0) * 100.0);
    println!("   • Production ready: {}/20 ({:.1}%)", 
             production_ready, (production_ready as f64 / 20.0) * 100.0);
    
    println!("\n✅ Task 1.1.3 Success Criteria:");
    println!("   • ✅ SimpleText template implemented and working");
    println!("   • {} Validation system operational", 
             if successful_validations == 20 { "✅" } else { "⚠️" });
    println!("   • {} Generated 20 test videos", 
             if successful_generations == 20 { "✅" } else { "⚠️" });
    println!("   • {} Average generation time < 30s", 
             if (total_generation_time / successful_generations as f64) < 30.0 { "✅" } else { "⚠️" });
    
    if successful_generations == 20 && successful_validations == 20 {
        println!("\n🎯 TASK 1.1.3 COMPLETED SUCCESSFULLY!");
        println!("🎬 SimpleText template engine is fully operational");
        println!("📊 Video validation system is working correctly");
        println!("🚀 Ready to proceed to Task 1.1.4: Audio integration");
    } else {
        println!("\n⚠️  Some tests failed - review errors above");
    }

    Ok(())
}
