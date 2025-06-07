use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: String,
    pub platform: String,
    pub schedule_type: String,
    pub template: String,
    pub title: String,
    pub background_color: String,
    pub text_color: String,
    pub audio_file: Option<String>,
    pub tags: String,
}

pub struct ContentSelector {
    pub content_items: Vec<ContentItem>,
    pub posted_history: HashSet<String>, // IDs of recently posted content
}

impl ContentSelector {
    pub fn new(content_items: Vec<ContentItem>) -> Self {
        Self {
            content_items,
            posted_history: HashSet::new(),
        }
    }

    // TODO: Implement content selection logic for Phase 1.2
    pub fn select_for_date(&self, _date: NaiveDate, _platform: &str) -> Option<ContentItem> {
        todo!("Content selection logic will be implemented in Phase 1.2")
    }
}
