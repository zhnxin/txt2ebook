mod chapter;
mod config;
mod template_type;
mod zip_utils;
pub use self::chapter::{ChapterInfo, MainChapter};
pub use self::config::Config;
pub use self::template_type::TemplateType;
pub use self::zip_utils::zip;
use rust_embed::RustEmbed;
#[warn(dead_code)]
use serde::Serialize;
#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct TemplateAssets;

#[derive(Serialize)]
pub struct BookInfo {
    title: String,
    cover: String,
    cover_source: String,
    author: String,
    descripration: String,
    chapter: Vec<chapter::ChapterInfo>,
    current_order: usize,
    uid: String,
    out_dir: String,
}
impl BookInfo {
    pub fn new(conf: &config::Config) -> Self {
        let cover = if conf.cover.ends_with(".png") || conf.cover.ends_with(".PNG") {
            String::from("cover.png")
        } else if !conf.cover.is_empty() {
            String::from("cover.jpg")
        } else {
            String::new()
        };
        Self {
            title: conf.title.clone(),
            cover: cover,
            cover_source: conf.cover.clone(),
            author: conf.author.clone(),
            descripration: String::new(),
            chapter: Vec::new(),
            current_order: 3,
            uid: hash_string(&conf.title),
            out_dir: String::new(),
        }
    }
    pub fn get_output_file(&self) -> String{
        std::fmt::format(format_args!(
            "{}{}",
            self.out_dir, self.title
        ))
    }

    pub fn get_out_dir(&self) -> &String {
        &self.out_dir
    }
    pub fn set_out_dir(&mut self, dir: &str) {
        if dir.is_empty() {
            return;
        }
        self.out_dir = String::from(dir);
        if !self.out_dir.ends_with("/") {
            self.out_dir.push('/');
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_uid(&self) -> &String {
        &self.uid
    }
    pub fn render_title(
        &mut self,
        descripration: String,
        dir_name: &String,
        template: &handlebars::Handlebars,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.descripration = descripration;
        let file = std::fs::File::create(std::fmt::format(format_args!(
            "{}/OEBPS/Xhtml/title.xhtml",
            dir_name
        )))?;
        template.render_to_write(TemplateType::Title.to_string().as_str(), self, file)?;
        Ok(())
    }
    pub fn render_left(
        &self,
        dir_name: &String,
        template: &handlebars::Handlebars,
    ) -> Result<(), Box<dyn std::error::Error>> {
        {
            let file = std::fs::File::create(std::fmt::format(format_args!(
                "{}/OEBPS/content.opf",
                dir_name
            )))?;
            template.render_to_write(TemplateType::Opf.to_string().as_str(), self, file)?;
        }
        {
            let file = std::fs::File::create(std::fmt::format(format_args!(
                "{}/OEBPS/Xhtml/catalog.xhtml",
                dir_name
            )))?;
            template.render_to_write(TemplateType::Catalog.to_string().as_str(), self, file)?;
        }
        {
            let file = std::fs::File::create(std::fmt::format(format_args!(
                "{}/OEBPS/toc.ncx",
                dir_name
            )))?;
            template.render_to_write(TemplateType::Ncx.to_string().as_str(), self, file)?;
        }
        Ok(())
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

pub fn prepare_book(
    dir_name: &String,
    template: &handlebars::Handlebars,
    book_info: &BookInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&std::fmt::format(format_args!("{}/META-INF", dir_name)))?;
    std::fs::create_dir_all(&std::fmt::format(format_args!("{}/OEBPS", dir_name)))?;
    std::fs::create_dir_all(&std::fmt::format(format_args!("{}/OEBPS/Xhtml", dir_name)))?;
    std::fs::create_dir_all(&std::fmt::format(format_args!("{}/OEBPS/Styles", dir_name)))?;
    std::fs::create_dir_all(&std::fmt::format(format_args!("{}/OEBPS/Images", dir_name)))?;
    std::fs::write(
        &std::fmt::format(format_args!("{}/mimetype", dir_name)),
        "application/epub+zip",
    )?;
    std::fs::write(
        &std::fmt::format(format_args!("{}/META-INF/container.xml", dir_name)),
        TemplateAssets::get("container.xml").unwrap().data.as_ref(),
    )?;
    std::fs::write(
        &std::fmt::format(format_args!("{}/OEBPS/Styles/stylesheet.css", dir_name)),
        TemplateAssets::get("stylesheet.css").unwrap().data.as_ref(),
    )?;
    if !book_info.get_cover().is_empty() {
        std::fs::copy(
            &book_info.cover_source,
            &std::fmt::format(format_args!(
                "{}/OEBPS/Images/{}",
                dir_name,
                book_info.get_cover()
            )),
        )?;
    }
    {
        let file = std::fs::File::create(&std::fmt::format(format_args!(
            "{}/OEBPS/Xhtml/cover.xhtml",
            dir_name
        )))?;
        template.render_to_write(TemplateType::Cover.to_string().as_str(), &book_info, file)?;
    }
    Ok(())
}

pub fn render_content(
    chapter_content: &MainChapter,
    dir_name: &String,
    template: &handlebars::Handlebars,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(std::fmt::format(format_args!(
        "{}/OEBPS/Xhtml/chap_{}.xhtml",
        dir_name,
        chapter_content.get_id()
    )))?;
    template.render_to_write(
        TemplateType::Content.to_string().as_str(),
        &chapter_content,
        file,
    )?;
    Ok(())
}

pub fn run_kindlegen(book_name: &String, source_file: &String) {
    if cfg!(target_os = "windows") {
        std::process::Command::new("kindlegen.exe")
            .args(&vec![
                "-dont_append_source",
                "-c1",
                "-o",
                format!("{}", format_args!("{}.mobi", book_name)).as_str(),
                source_file,
            ])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("failed to execute kindlegen")
    } else {
        std::process::Command::new("kindlegen")
            .args(&vec![
                "-dont_append_source",
                "-c1",
                "-o",
                format!("{}", format_args!("{}.mobi", book_name)).as_str(),
                source_file,
            ])
            .stderr(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .output()
            .expect("failed to execute kindlegen")
    };
}
