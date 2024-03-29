extern crate rust_embed;
use clap::{App, Arg};
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("txt2ebookindlegen with rust")
        .author("zhngxin@aliyun.com")
        .about("convert text file to kindlegen format")
        .arg(
            Arg::with_name("kindlegen")
                .short('k')
                .long("kindlegen")
                .help("run kindlegen after epub file generated"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("debug not to delete tmp file"),
        )
        .arg(
            Arg::with_name("dir")
                .long("dir")
                .value_name("DIR")
                .short('d')
                .help("output dir path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("generateconfig")
                .short('g')
                .long("generate-config")
                .help("generate example config file"),
        )
        .arg(
            Arg::with_name("config")
                .short('c')
                .value_name("FILE")
                .help("Set a custom toml config file")
                .default_value(&"config.toml")
                .takes_value(true),
        )
        .get_matches();
    if matches.is_present("generateconfig") {
        std::fs::write(
            matches.value_of("config").unwrap_or("config.toml"),
            txt2ebook::TemplateAssets::get("config.toml.tmpl")
                .unwrap()
                .data
                .as_ref(),
        )?;
        return Ok(());
    }
    let config = txt2ebook::Config::from(matches.value_of("config").unwrap_or("config.toml"))?;
    let chapter_regex = regex::Regex::new(&config.chapter)?;
    let mut is_use_subchapter = false;
    let subchapter_regex = if config.subchapter.is_empty() {
        regex::Regex::new(&r"^\s+$")?
    } else {
        is_use_subchapter = true;
        regex::Regex::new(&config.subchapter)?
    };
    let blink_regex = regex::Regex::new(&r"^\s*$")?;

    let mut book_info = txt2ebook::BookInfo::new(&config);
    if let Some(dir) = matches.value_of("dir") {
        if !dir.is_empty() {
            book_info.set_out_dir(dir);
        }
    }
    let mut template_reg = handlebars::Handlebars::new();
    template_reg.register_escape_fn(handlebars::no_escape);
    for tmp_type in txt2ebook::TemplateType::VALUES.iter() {
        template_reg.register_template_string(
            tmp_type.to_string().as_str(),
            std::str::from_utf8(
                txt2ebook::TemplateAssets::get(tmp_type.get_file_name())
                    .unwrap()
                    .data
                    .as_ref(),
            )?,
        )?;
    }
    // create output dir
    let dir_name = txt2ebook::hash_string(book_info.get_title());
    txt2ebook::prepare_book(&dir_name, &template_reg, &book_info)?;

    let mut chapter_content = txt2ebook::MainChapter::new(String::new(), 3);

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
                book_info.render_title(
                    chapter_content.get_chapter_content(),
                    &dir_name,
                    &template_reg,
                )?;
            } else {
                txt2ebook::render_content(&chapter_content, &dir_name, &template_reg)?;
            }
            book_info.add_chapter(line_str.clone());
            chapter_content.restore(line_str, book_info.get_current_order());
        } else if is_use_subchapter && subchapter_regex.is_match(&line_str) {
            book_info.add_subchapter(line_str.clone());
            chapter_content.add_subchapter(line_str, book_info.get_current_order());
        } else {
            chapter_content.append(&line_str, config.is_html_p);
        }
    }
    if !chapter_content.is_empty() {
        txt2ebook::render_content(&chapter_content, &dir_name, &template_reg)?;
        chapter_content.restore(String::new(), 0);
    }
    book_info.render_left(&dir_name, &template_reg)?;
    let epub_file_name = book_info.get_output_file() + ".epub";
    txt2ebook::zip_book(&dir_name, &epub_file_name)?;
    if !matches.is_present("debug") {
        std::fs::remove_dir_all(&dir_name)?;
    }
    if matches.is_present("kindlegen") {
        txt2ebook::run_kindlegen(&config.title, &epub_file_name);
    }
    Ok(())
}
