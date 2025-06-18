# Automated Video Generation & Posting System

An AI-driven system for generating and posting short-form videos to social media platforms automatically. Built with Rust for performance and reliability.

<details>
<summary>Project Status</summary>

**Current Phase**: Foundation & Local Development (Phase 1)  
**Progress**: 60% Complete (3/5 core tasks finished)  
**Last Updated**: June 7, 2025

### Completed Tasks

- **Task 1.1.1**: FFmpeg wrapper setup & basic video generation
- **Task 1.1.2**: Text overlay system with font rendering
- **Task 1.1.3**: Template engine with comprehensive validation

### Current Work

- **Task 1.1.4**: Audio integration (next)

### Upcoming Tasks

- **Task 1.2.1**: CSV parser for content management
- **Task 1.2.2**: Content selection algorithm

</details>

<details>
<summary>Quick Start</summary>

### Prerequisites

- Rust (latest stable)
- FFmpeg with development headers
- Linux environment (tested on Ubuntu)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd automate-social-media-posts

# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y ffmpeg libavcodec-dev libavformat-dev libavfilter-dev libswscale-dev

# Build the project
cargo build --release
```

### Running Tests

```bash
# Run comprehensive template engine tests (generates 20 test videos)
cargo run --bin test_template_engine

# Run basic text overlay tests
cargo run --bin test_text_overlay

# Run the main application
cargo run
```

</details>

<details>
<summary>Current Capabilities</summary>

## Current Capabilities

### Video Generation

- **Format**: MP4 (H.264) at 1080x1920 (9:16 aspect ratio)
- **Frame Rate**: 30fps with 2 Mbps bitrate
- **Duration**: 10-60 seconds (configurable)
- **Templates**: SimpleText (more coming soon)

### Text Rendering

- **Font Support**: Embedded DejaVu Sans TTF
- **Colors**: Full hex color support for text and backgrounds
- **Sizing**: Flexible font sizes (24px-80px tested)
- **Positioning**: Automatic centering (horizontal and vertical)

### Quality Validation

- **Dimension Validation**: Ensures 1080x1920 output
- **File Size Limits**: <50MB per video
- **Format Compliance**: MP4/H.264 verification
- **Playability Testing**: Automated validation

</details>

<details>
<summary>Technical Architecture</summary>

## Technical Architecture

### Core Components

```rust
// Video generation pipeline
VideoSpec -> VideoGenerator -> MP4 Output -> VideoValidator

// Key structures
VideoSpec {
    title: String,
    template: VideoTemplate,
    duration_seconds: u32,
    background_color: String,  // hex
    text_color: String,        // hex  
    font_size: u32,
    audio_track: Option<String>,
}
```

### Project Structure

```
src/
├── video/
│   ├── generator.rs     # Core video generation logic
│   ├── validation.rs    # Quality validation system
│   └── mod.rs          # Video module types and exports
├── assets/
│   └── DejaVuSans.ttf  # Embedded font
├── bin/
│   ├── test_template_engine.rs  # Comprehensive test suite
│   └── test_text_overlay.rs     # Text overlay validation
└── [config, content, platforms]  # Future modules
```

</details>

<details>
<summary>Performance Metrics</summary>

## Performance Metrics

**Latest Test Results** (20 video generation test):
- **Success Rate**: 100% (20/20 videos)
- **Average Generation Time**: 8.6 seconds
- **File Size Range**: 111KB - 1.1MB
- **Total Test Duration**: 173 seconds

</details>

<details>
<summary>Generated Video Examples</summary>

## Generated Video Examples

The system currently generates videos with:
- Various color combinations (high contrast, pastels, bright colors)
- Multiple font sizes and text lengths
- Different durations (10-60 seconds)
- Consistent 9:16 aspect ratio for mobile platforms

</details>

<details>
<summary>Development</summary>

## Development

### Building

```bash
# Debug build
cargo build

# Release build (recommended for video generation)
cargo build --release

# Run specific tests
cargo run --bin test_template_engine
```

### Adding New Templates

Templates are defined in `src/video/mod.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoTemplate {
    SimpleText,        // Implemented
    TitleCard,         // Planned
    Slideshow { slides: Vec<String> },  // Planned
}
```

</details>

<details>
<summary>Configuration</summary>

## Configuration

Videos are configured using the `VideoSpec` structure:

```rust
let spec = VideoSpec {
    title: "My Video Title".to_string(),
    template: VideoTemplate::SimpleText,
    duration_seconds: 30,
    background_color: "#1a1a1a".to_string(),
    text_color: "#ffffff".to_string(),  
    font_size: 48,
    audio_track: None,  // Coming in Task 1.1.4
};
```

</details>

<details>
<summary>Upcoming Features</summary>

## Upcoming Features

- **Audio Integration**: Background music and sound effects
- **CSV Content Management**: Automated content scheduling
- **Platform APIs**: YouTube Shorts and TikTok integration
- **Advanced Templates**: Title cards, slideshows, animations
- **GitHub Actions**: Automated daily posting

</details>

<details>
<summary>Known Issues</summary>

## Known Issues

- **Duration Metadata**: Videos contain correct frames but metadata shows ~0.004s instead of specified duration
- **Audio Support**: Not yet implemented (Task 1.1.4)

</details>

<details>
<summary>Documentation</summary>

## Documentation

- [CHANGELOG.md](CHANGELOG.md) - Detailed development history
- [PRD Documentation] - Comprehensive project requirements (see codebase)
- Release notes are automatically generated by [Release Drafter](https://github.com/marketplace/actions/release-drafter).

</details>

<details>
<summary>Contributing</summary>

## Contributing

This project follows the phased development approach outlined in the PRD:

1. **Phase 1**: Foundation & Local Development (60% complete)
2. **Phase 2**: Mock Integration & Testing  
3. **Phase 3**: Real Platform Integration

- Label pull requests with `feat`, `fix`, `docs`, or `chore` for automated release notes.
</details>

<details>
<summary>License</summary>

## License

[License information to be added]

---

**Built with Rust, FFmpeg, and a focus on reliability and performance.**

