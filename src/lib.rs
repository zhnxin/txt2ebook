pub mod chapter;
pub mod config;
pub mod template_type;
pub mod zip_utils;
#[warn(dead_code)]
use serde::Serialize;
#[derive(Serialize)]
pub struct BookInfo {
    title: String,
    cover: String,
    author: String,
    descripration: String,
    chapter: Vec<chapter::ChapterInfo>,
    current_order: usize,
}
impl BookInfo {
    pub fn new(conf: &config::Config) -> Self {
        let cover = if conf.cover.ends_with(".png") || conf.cover.ends_with(".PNG") {
            String::from("cover.png")
        } else {
            String::from("cover.jpg")
        };
        Self {
            title: conf.title.clone(),
            cover: cover,
            author: conf.author.clone(),
            descripration: String::new(),
            chapter: Vec::new(),
            current_order: 3,
        }
    }
    pub fn get_cover(&self) -> &String {
        &self.cover
    }
    pub fn is_chapter_empty(&self) -> bool {
        self.chapter.is_empty()
    }

    pub fn add_chapter(&mut self, title: String) {
        self.current_order += 1;
        self.chapter
            .push(chapter::ChapterInfo::new(title, self.current_order))
    }

    pub fn add_subchapter(&mut self, title: String) {
        self.current_order += 1;
        if let Some(last) = self.chapter.last_mut() {
            last.add_subchapter(title, self.current_order);
        }
    }

    pub fn get_current_order(&self) -> usize {
        self.current_order
    }

    pub fn set_descripration(&mut self, descripration: String) {
        self.descripration = descripration;
    }
}

pub fn hash_string(s: &String) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    Hash::hash(s, &mut hasher);
    std::fmt::format(format_args!("{}", hasher.finish()))
}

pub fn zip_book(dir_name: &str, output_file: &str) -> zip::result::ZipResult<()> {
    zip_utils::zip(dir_name, output_file, zip::CompressionMethod::Deflated)
}
