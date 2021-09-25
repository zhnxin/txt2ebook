extern crate rust_embed;
use clap::{App, Arg};
use rust_embed::RustEmbed;
use std::fs::{File, OpenOptions};
use txt4k::template_type::TemplateType;
#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("txt4kindlegen with rust")
        .author("zhngxin@aliyun.com")
        .about("convert text file to kindlegen format")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom toml config file")
                .default_value(&"config.toml")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("notkindlegen")
                .short("k")
                .long("not-kindlegen")
                .help("disable run kindlegen"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("debug not to delete tmp file"),
        )
        .get_matches();
    let config = txt4k::config::Config::from(matches.value_of("config").unwrap_or("config.toml"))?;
    let chapter_regex = regex::Regex::new(&config.chapter)?;
    let mut is_use_subchapter = false;
    let subchapter_regex = if config.subchapter.is_empty() {
        regex::Regex::new(&r"^\s+$")?
    } else {
        is_use_subchapter = true;
        regex::Regex::new(&config.subchapter)?
    };
    let blink_regex = regex::Regex::new(&r"^\s*$")?;

    let mut book_info = txt4k::BookInfo::new(&config);
    let mut template_reg = handlebars::Handlebars::new();
    template_reg.register_escape_fn(handlebars::no_escape);
    for tmp_type in txt4k::template_type::TemplateType::VALUES.iter() {
        template_reg.register_template_string(
            tmp_type.to_string().as_str(),
            std::str::from_utf8(
                Templates::get(tmp_type.get_file_name())
                    .unwrap()
                    .data
                    .as_ref(),
            )?,
        )?;
    }
    // create output dir
    let dir_name = txt4k::hash_string(&config.title);
    {
        std::fs::create_dir_all(&std::fmt::format(format_args!("{}/META-INF", dir_name)))?;
        std::fs::create_dir_all(&std::fmt::format(format_args!("{}/OEBPS", dir_name)))?;
        std::fs::write(
            &std::fmt::format(format_args!("{}/mimetype", dir_name)),
            "application/epub+zip",
        )?;
        std::fs::write(
            &std::fmt::format(format_args!("{}/META-INF/container.xml", dir_name)),
            Templates::get("container.xml").unwrap().data.as_ref(),
        )?;
        std::fs::write(
            &std::fmt::format(format_args!("{}/OEBPS/stylesheet.css", dir_name)),
            Templates::get("stylesheet.css").unwrap().data.as_ref(),
        )?;
        let file = File::create(&std::fmt::format(format_args!(
            "{}/OEBPS/cover.xhtml",
            dir_name
        )))?;
        std::fs::copy(
            config.cover,
            &std::fmt::format(format_args!("{}/OEBPS/{}", dir_name, book_info.get_cover())),
        )?;
        template_reg.render_to_write(
            txt4k::template_type::TemplateType::Cover
                .to_string()
                .as_str(),
            &book_info,
            file,
        )?;
    }

    let mut chapter_content = txt4k::chapter::MainChapter::new(String::new(),4);

    let text_file = OpenOptions::new().read(true).open(config.file)?;
    let encoding_format =
        encoding::label::encoding_from_whatwg_label(&config.encoding).expect("unknow encoding");
    let bufreader = encodingbufreader::BufReaderEncoding::new(text_file, encoding_format);
    for line in bufreader.lines() {
        let line_str = line?;
        if blink_regex.is_match(&line_str) {
            continue;
        } else if chapter_regex.is_match(&line_str) {
            if book_info.is_chapter_empty() {
                book_info.set_descripration(chapter_content.get_chapter_content());
                let file = File::create(std::fmt::format(format_args!(
                    "{:}/OEBPS/title.xhtml",
                    dir_name
                )))?;
                template_reg.render_to_write(
                    TemplateType::Title.to_string().as_str(),
                    &book_info,
                    file,
                )?;
            } else {
                let file = File::create(std::fmt::format(format_args!(
                    "{}/OEBPS/chap_{}.html",
                    dir_name,
                    chapter_content.get_id()
                )))?;
                template_reg.render_to_write(
                    TemplateType::Content.to_string().as_str(),
                    &chapter_content,
                    file,
                )?;
            }
            book_info.add_chapter(line_str.clone());
            chapter_content.restore(line_str,book_info.get_current_order());
        } else if is_use_subchapter && subchapter_regex.is_match(&line_str) {
            book_info.add_subchapter(line_str.clone());
            chapter_content.add_subchapter(line_str, book_info.get_current_order());
        } else {
            chapter_content.append(&line_str, config.is_html_p);
        }
    }
    if !chapter_content.is_empty() {
        let file = File::create(std::fmt::format(format_args!(
            "{}/OEBPS/chap_{}.html",
            dir_name,
            chapter_content.get_id()
        )))?;
        template_reg.render_to_write(
            TemplateType::Content.to_string().as_str(),
            &chapter_content,
            file,
        )?;
        chapter_content.restore(String::new(),0);
    }
    let book_info_data = serde_json::json!(book_info);
    {
        let file = File::create(std::fmt::format(format_args!(
            "{}/OEBPS/content.opf",
            dir_name
        )))?;
        template_reg.render_to_write(
            TemplateType::Opf.to_string().as_str(),
            &book_info_data,
            file,
        )?;
    }
    {
        let file = File::create(std::fmt::format(format_args!(
            "{}/OEBPS/catalog.html",
            dir_name
        )))?;
        template_reg.render_to_write(
            TemplateType::Catalog.to_string().as_str(),
            &book_info_data,
            file,
        )?;
    }

    {
        let file = File::create(std::fmt::format(format_args!("{}/OEBPS/toc.ncx", dir_name)))?;
        template_reg.render_to_write(
            TemplateType::Ncx.to_string().as_str(),
            &book_info_data,
            file,
        )?;
    }
    let epub_file_name = std::fmt::format(format_args!("{}.epub", &config.title));
    txt4k::zip_book(&dir_name, &epub_file_name)?;
    if !matches.is_present("debug") {
        std::fs::remove_dir_all(&dir_name)?;
    }
    if !matches.is_present("notkindlegen") {
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("kindlegen.exe")
                .args(&vec![
                    "-dont_append_source",
                    "-c1",
                    "-o",
                    format!("{}", format_args!("{}.mobi", &config.title)).as_str(),
                    &epub_file_name,
                ])
                .output()
                .expect("failed to execute kindlegen")
        } else {
            std::process::Command::new("kindlegen")
                .args(&vec![
                    "-dont_append_source",
                    "-c1",
                    "-o",
                    format!("{}", format_args!("{}.mobi", &config.title)).as_str(),
                    &epub_file_name,
                ])
                .output()
                .expect("failed to execute kindlegen")
        };
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}
