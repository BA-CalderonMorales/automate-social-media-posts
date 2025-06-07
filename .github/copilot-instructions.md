# Automated Video Generation & Posting System 

## Executive Summary

This document outlines a phased approach to building an automated system that generates and posts short-form videos to social media platforms. The system prioritizes testability, incremental development, and clear validation criteria to enable AI-assisted development with minimal human iteration.

## Project Goals

### Primary Objectives
- Generate 15-60 second vertical videos programmatically
- Post content to YouTube Shorts and TikTok on a daily schedule
- Maintain content variety through CSV-driven configuration
- Enable easy platform extension without code restructuring

### Success Metrics
- System runs reliably for 30 consecutive days without manual intervention
- Generated videos meet platform technical requirements (dimensions, format, duration)
- Zero failed uploads due to system errors (API/network failures are acceptable)
- New platform integration takes <4 hours of development time

## System Architecture

### Core Components

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Content CSV   │    │   Video Generator │    │ Platform APIs   │
│                 │    │                  │    │                 │
│ - Schedule      │───▶│ - Template Engine │───▶│ - YouTube       │
│ - Metadata      │    │ - FFmpeg Wrapper │    │ - TikTok        │
│ - Templates     │    │ - Validation     │    │ - Mock Outputs  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌──────────────────┐
                    │ GitHub Actions   │
                    │ Scheduler        │
                    └──────────────────┘
```

### Technology Stack
- **Runtime**: Rust (latest stable)
- **Video Processing**: FFmpeg via `ffmpeg-next` crate
- **HTTP Client**: `reqwest` for API calls
- **CSV Processing**: `csv` + `serde` for data handling
- **Scheduling**: GitHub Actions (cron)
- **Configuration**: TOML files for settings

## Phase 1: Foundation & Local Development

### 1.1 Video Generation Engine

**Objective**: Create a reliable video generator that produces platform-compliant content.

**Video Specifications**:
```rust
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
```

**Technical Requirements**:
- Output format: MP4 (H.264 + AAC)
- Dimensions: 1080x1920 (9:16 aspect ratio)
- Duration: 10-60 seconds
- File size: <50MB
- Frame rate: 30fps

**Validation Criteria**:
```rust
#[derive(Debug)]
pub struct VideoValidation {
    pub correct_dimensions: bool,
    pub duration_in_range: bool,
    pub file_size_under_limit: bool,
    pub has_audio: bool,
    pub is_playable: bool,
}
```

**Success Criteria**:
- Generates 10 different videos with 100% validation pass rate
- Average generation time <30 seconds
- All videos play correctly in VLC media player
- File sizes are reasonable (5-20MB for 15-30 second videos)

**Implementation Tasks**:
```
Task 1.1.1: FFmpeg wrapper setup
- Install and configure FFmpeg dependencies
- Create safe Rust wrapper for basic operations
- Test: Generate solid color video, verify output

Task 1.1.2: Text overlay system
- Implement text rendering on video frames
- Support custom fonts, sizes, colors, positioning
- Test: Generate videos with different text configurations

Task 1.1.3: Template engine
- Implement SimpleText template (text on colored background)
- Add validation for all generated videos
- Test: Generate 20 videos, verify all pass validation

Task 1.1.4: Audio integration
- Add background music/audio track support
- Ensure audio-video sync
- Test: Generate videos with/without audio
```

### 1.2 Content Management System

**CSV Structure**:
```csv
id,platform,schedule_type,template,title,background_color,text_color,audio_file,tags
1,youtube,daily,SimpleText,"Daily Coding Tip #1",#1a1a1a,#ffffff,music/upbeat.mp3,"coding,tips"
2,tiktok,daily,TitleCard,"Quick Python Trick",#ff6b6b,#ffffff,,python,tricks
3,youtube,weekly,Slideshow,"Weekly Recap",#4ecdc4,#2c3e50,music/calm.mp3,"recap,weekly"
```

**Content Selection Logic**:
```rust
pub struct ContentSelector {
    pub content_items: Vec<ContentItem>,
    pub posted_history: HashSet<String>, // IDs of recently posted content
}

impl ContentSelector {
    pub fn select_for_date(&self, date: Date, platform: Platform) -> Option<ContentItem> {
        // Priority: daily > weekly > monthly
        // Avoid duplicates from last 7 days
        // Ensure platform match
    }
}
```

**Success Criteria**:
- Parse CSV with 50+ rows without errors
- Correctly select content based on date and platform
- Never select the same content twice within 7 days
- Handle malformed CSV gracefully with clear error messages

**Implementation Tasks**:
```
Task 1.2.1: CSV parser
- Define ContentItem struct with all required fields
- Implement parsing with comprehensive error handling
- Test: Parse valid/invalid CSV files, verify error messages

Task 1.2.2: Content selection algorithm
- Implement daily/weekly/monthly scheduling logic
- Add duplicate prevention with configurable lookback period
- Test: Simulate 30 days of selections, verify no duplicates

Task 1.2.3: Content validation
- Validate required fields, color formats, file paths
- Provide helpful error messages for common mistakes
- Test: Validate 20 content items with various error conditions
```

### 1.3 Platform Abstraction Layer

**Interface Design**:
```rust
#[async_trait]
pub trait VideoPlatform {
    async fn upload_video(
        &self,
        video_path: &Path,
        metadata: VideoMetadata,
    ) -> Result<UploadResult, PlatformError>;
    
    fn platform_name(&self) -> &str;
    fn max_file_size(&self) -> u64;
    fn supported_formats(&self) -> Vec<String>;
}

#[derive(Debug)]
pub struct VideoMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub privacy: PrivacyLevel,
}
```

**Mock Implementation**:
```rust
pub struct MockPlatform {
    pub name: String,
    pub upload_log: Arc<Mutex<Vec<UploadLog>>>,
}

impl VideoPlatform for MockPlatform {
    async fn upload_video(&self, video_path: &Path, metadata: VideoMetadata) -> Result<UploadResult, PlatformError> {
        // Validate file exists and is readable
        // Log upload attempt with timestamp
        // Return success after configurable delay
    }
}
```

**Success Criteria**:
- Mock platforms log all upload attempts with correct metadata
- File validation catches common issues (missing files, wrong format)
- Platform-specific constraints are enforced
- Easy to swap mock implementations with real ones

## Phase 2: Mock Integration & Testing

### 2.1 End-to-End Integration

**System Configuration**:
```toml
# config.toml
[video]
output_directory = "output/"
temp_directory = "temp/"
cleanup_after_upload = true

[scheduling]
timezone = "America/Chicago"
daily_post_time = "17:00"

[platforms.youtube]
enabled = false
upload_as_private = true
max_daily_uploads = 5

[platforms.tiktok]
enabled = false
max_daily_uploads = 10

[platforms.mock]
enabled = true
simulate_failures = false
```

**Integration Flow**:
1. Load configuration and content CSV
2. Select content for current date/time
3. Generate video using selected content
4. Validate generated video
5. Upload to enabled platforms
6. Log results and cleanup

**Success Criteria**:
- Complete end-to-end run takes <5 minutes
- Generates comprehensive logs for debugging
- Handles all error conditions gracefully
- Produces consistent results across multiple runs

### 2.2 GitHub Actions Integration

**Workflow Structure**:
```yaml
# .github/workflows/video-automation.yml
name: Daily Video Automation

on:
  schedule:
    - cron: "0 23 * * *"  # 5 PM CST (11 PM UTC)
  workflow_dispatch:       # Manual trigger for testing
    inputs:
      test_mode:
        description: 'Run in test mode'
        required: false
        default: 'true'

jobs:
  generate-and-post:
    runs-on: ubuntu-latest
    timeout-minutes: 15
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y ffmpeg libavcodec-dev libavformat-dev
          
      - name: Build project
        run: cargo build --release
        
      - name: Run automation
        env:
          YOUTUBE_CLIENT_ID: ${{ secrets.YOUTUBE_CLIENT_ID }}
          YOUTUBE_CLIENT_SECRET: ${{ secrets.YOUTUBE_CLIENT_SECRET }}
          YOUTUBE_REFRESH_TOKEN: ${{ secrets.YOUTUBE_REFRESH_TOKEN }}
          TIKTOK_CLIENT_KEY: ${{ secrets.TIKTOK_CLIENT_KEY }}
          TIKTOK_CLIENT_SECRET: ${{ secrets.TIKTOK_CLIENT_SECRET }}
          TIKTOK_ACCESS_TOKEN: ${{ secrets.TIKTOK_ACCESS_TOKEN }}
          TEST_MODE: ${{ github.event.inputs.test_mode || 'false' }}
        run: cargo run --release
        
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: automation-logs
          path: |
            logs/
            output/
```

**Testing Strategy**:
- Manual trigger with test mode enabled
- Dry-run mode that generates videos but doesn't upload
- Comprehensive logging for debugging failures
- Artifact upload for manual inspection

## Phase 3: Real Platform Integration

### 3.1 YouTube Integration

**Authentication Flow**:
```rust
pub struct YouTubeUploader {
    client: reqwest::Client,
    access_token: String,
    refresh_token: String,
    client_id: String,
    client_secret: String,
}

impl YouTubeUploader {
    pub async fn refresh_access_token(&mut self) -> Result<(), YouTubeError> {
        // Implement OAuth refresh flow
        // Update self.access_token
    }
    
    pub async fn upload_video_resumable(
        &self,
        video_path: &Path,
        metadata: VideoMetadata,
    ) -> Result<UploadResult, YouTubeError> {
        // Implement resumable upload protocol
        // Handle chunked uploads for large files
    }
}
```

**API Integration Requirements**:
- YouTube Data API v3 enabled
- OAuth 2.0 client credentials configured
- Resumable upload implementation for reliability
- Rate limit handling (quota: 10,000 units/day)
- Proper error handling for common API errors

**Testing Strategy**:
- Start with private uploads to test channel
- Use YouTube API Explorer for manual testing
- Implement comprehensive error logging
- Test with various video sizes and metadata

### 3.2 TikTok Integration

**API Challenges & Solutions**:
- **Business Verification Required**: Document process for obtaining API access
- **Chunked Upload Complexity**: Implement robust chunked upload with retry logic
- **Rate Limits**: Respect API quotas and implement backoff strategies

```rust
pub struct TikTokUploader {
    client: reqwest::Client,
    client_key: String,
    client_secret: String,
    access_token: String,
}

impl TikTokUploader {
    pub async fn init_upload(&self, video_size: u64) -> Result<InitUploadResponse, TikTokError> {
        // Initialize upload session
    }
    
    pub async fn upload_video_chunked(
        &self,
        video_path: &Path,
        upload_id: &str,
    ) -> Result<(), TikTokError> {
        // Implement chunked upload with proper error handling
    }
    
    pub async fn complete_upload(
        &self,
        upload_id: &str,
        metadata: VideoMetadata,
    ) -> Result<UploadResult, TikTokError> {
        // Finalize upload and publish
    }
}
```

**Fallback Strategy**:
If TikTok API access is not available:
- Implement file-based output for manual upload
- Generate upload instructions/scripts
- Plan for future API integration when available

## Implementation Roadmap

### Week 1-2: Foundation
- [ ] Set up Rust project structure
- [ ] Implement basic video generation (SimpleText template)
- [ ] Create CSV parser and content selector
- [ ] Add comprehensive testing suite

### Week 3-4: Integration
- [ ] Implement mock platform uploaders
- [ ] Create end-to-end integration flow
- [ ] Set up GitHub Actions workflow
- [ ] Test complete pipeline with mocks

### Week 5-6: YouTube Integration
- [ ] Implement YouTube OAuth flow
- [ ] Add resumable upload capability
- [ ] Test with development YouTube channel
- [ ] Handle rate limits and error cases

### Week 7-8: TikTok Integration
- [ ] Apply for TikTok API access
- [ ] Implement chunked upload flow
- [ ] Add comprehensive error handling
- [ ] Test with development TikTok account

### Week 9-10: Production & Monitoring
- [ ] Deploy to production schedule
- [ ] Add monitoring and alerting
- [ ] Create operational documentation
- [ ] Plan for future platform extensions

## Risk Mitigation

### Technical Risks
1. **FFmpeg Installation Issues**: Provide Docker alternative for consistent environment
2. **API Rate Limits**: Implement exponential backoff and quota monitoring
3. **Video Generation Failures**: Add retry logic and fallback templates
4. **GitHub Actions Reliability**: Add manual trigger capability and local execution option

### Operational Risks
1. **Content Moderation**: Start with safe, generic content templates
2. **API Access Denial**: Implement file-based fallbacks for manual upload
3. **Quota Exhaustion**: Monitor usage and implement alerts
4. **Account Suspension**: Use dedicated accounts for automation

## Success Validation

### Automated Tests
- Unit tests for each component (>80% coverage)
- Integration tests for end-to-end flow
- Video validation tests (format, dimensions, duration)
- CSV parsing and content selection tests

### Manual Validation
- Weekly review of generated content quality
- Monthly analysis of posting consistency
- Quarterly review of platform performance metrics
- Annual review of system reliability and improvements

### Monitoring & Alerts
- Daily health checks via GitHub Actions
- Email notifications for critical failures
- Weekly summary reports of activity
- Monthly analytics on content performance

## Future Extensions

### Additional Platforms
- **Instagram Reels**: Similar to TikTok integration
- **LinkedIn Video**: Professional content focus
- **Twitter/X Video**: Shorter duration content
- **Facebook Reels**: Meta ecosystem integration

### Enhanced Features
- **AI-Generated Content**: Integration with GPT/Claude for dynamic content
- **Advanced Templates**: Multi-slide presentations, animations
- **Analytics Integration**: Performance tracking and optimization
- **A/B Testing**: Template and timing optimization

### Infrastructure Improvements
- **Cloud Functions**: Migration from GitHub Actions for better reliability
- **Database Integration**: Replace CSV with proper database
- **Web Dashboard**: UI for content management and monitoring
- **Mobile App**: Remote monitoring and manual triggers

## Conclusion

This PRD provides a comprehensive, phased approach to building a reliable video automation system. By starting with solid foundations, implementing thorough testing, and gradually adding complexity, we minimize risk while ensuring each component can be validated independently.

The modular architecture and clear interfaces make this system ideal for AI-assisted development, where each task has well-defined inputs, outputs, and success criteria. The extensive testing strategy ensures quality while the incremental approach allows for course correction as we learn from each phase.

Success depends on disciplined execution of each phase, thorough testing at every step, and maintaining the principle that every component should be independently testable and replaceable.

## Text Formatting Rules

### NO EMOJIS
- **NEVER use emojis in any code, documentation, comments, or generated text**
- This includes but is not limited to: 📊 🚀 ✅ ❌ 💡 🔧 📝 🎯 ⚡ 🌟 🎨 📱 💻 🔥 ⭐
- Use plain text descriptors instead (e.g., "Status" instead of "📊 Status")
- This applies to README files, CHANGELOG files, commit messages, and all documentation

### Preferred Alternatives
Instead of emojis, use:
- **Bold text** for emphasis
- `Code formatting` for technical terms
- Clear, descriptive section headers
- Bullet points for lists
- Numbered lists for sequences

## Documentation Style

### Section Organization
- Use `<details><summary>` tags for collapsible sections in Markdown
- Keep section headers clear and descriptive
- Maintain consistent formatting throughout files
- Use proper Markdown hierarchy (h1, h2, h3, etc.)

### Code Comments
- Write clear, descriptive comments without emojis
- Use TODO/FIXME/NOTE prefixes for special comments
- Maintain professional tone in all documentation

## File Naming and Structure
- Use lowercase with hyphens for file names
- Keep directory structure logical and organized
- Maintain consistency with existing patterns

## Commit Messages
- Use conventional commit format when applicable
- Write clear, descriptive messages without emojis
- Focus on the actual changes made

## These instructions should be followed at all times when generating content for this workspace.