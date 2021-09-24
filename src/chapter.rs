#[warn(dead_code)]
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ChapterContent {
    is_sub_cha: bool,
    title: String,
    content: String,
}
#[derive(Serialize, Debug)]
pub struct ChapterMeta {
    title: String,
    is_sub_cha: bool,
    has_sub_chap: bool,
    id: isize,
    order: isize,
}
#[derive(Serialize, Debug)]
pub struct ChapterInfo {
    chapter: ChapterMeta,
    subchapter: Vec<ChapterMeta>,
}

impl ChapterInfo {
    pub fn new(title: String, order: isize) -> Self {
        Self {
            chapter: ChapterMeta {
                title: title,
                is_sub_cha: false,
                has_sub_chap: false,
                id: order,
                order: order,
            },
            subchapter: Vec::new(),
        }
    }

    pub fn add_subchapter(&mut self, title: String, order: isize) {
        self.chapter.has_sub_chap = true;
        self.subchapter.push(ChapterMeta {
            has_sub_chap: false,
            title: title,
            is_sub_cha: true,
            id: order,
            order: order,
        })
    }
}

impl ChapterContent {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            is_sub_cha: false,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    pub fn clear(&mut self) {
        self.content.clear();
    }
    pub fn append(&mut self, content: &String, is_label_p: bool) {
        if is_label_p {
            self.content.push_str("<p>");
            self.content.push_str(content.trim());
            self.content.push_str("</p>\n");
        } else {
            self.content.push_str(content);
            self.content.push('\n');
        }
        self.content.push_str("<br/>");
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn restore(&mut self, title: String, is_sub_cha: bool) {
        self.title = title;
        self.content.clear();
        self.is_sub_cha = is_sub_cha;
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    pub fn get_is_sub_cha(&self) -> bool {
        self.is_sub_cha
    }
}
