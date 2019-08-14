extern crate rust_embed;
mod chapter;
mod config;
use clap::{App, Arg};
use rust_embed::RustEmbed;
use std::fs::{File, OpenOptions};
use std::io::prelude::Write;
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
        .get_matches();
    let config = config::Config::from(matches.value_of("config").unwrap_or("config.toml"))?;
    let chapter_regex = regex::Regex::new(config.get_chapter())?;
    let mut is_use_subchapter = false;
    let subchapter_regex = if config.get_subchapter().is_empty() {
        regex::Regex::new(&r"^\s+$")?
    } else {
        is_use_subchapter = true;
        regex::Regex::new(config.get_subchapter())?
    };
    let blink_regex = regex::Regex::new(&r"^\s*$")?;
    let mut template_reg = handlebars::Handlebars::new();
    let opf_name = "opf";
    let index_name = "index";
    let ncx_name = "ncx";
    let toc_name = "toc";

    template_reg.register_template_string(
        &opf_name,
        std::str::from_utf8(Templates::get("book.opf").unwrap().as_ref())?,
    )?;
    template_reg.register_template_string(
        &index_name,
        std::str::from_utf8(Templates::get("index.html").unwrap().as_ref())?,
    )?;
    template_reg.register_template_string(
        &ncx_name,
        std::str::from_utf8(Templates::get("toc.ncx").unwrap().as_ref())?,
    )?;
    template_reg.register_template_string(
        &toc_name,
        std::str::from_utf8(Templates::get("toc.xhtml").unwrap().as_ref())?,
    )?;
    {
        let file = File::create("book.opf")?;
        template_reg.render_to_write(&opf_name, &config, file)?;
    }
    {
        let file = File::create("index.html")?;
        template_reg.render_to_write(&index_name, &config, file)?;
    }
    let text_file = OpenOptions::new().read(true).open(config.get_file())?;
    let encoding_format = encoding::label::encoding_from_whatwg_label(config.get_encoding())
        .expect("unknow encoding");
    let bufreader = encodingbufreader::BufReaderEncoding::new(text_file, encoding_format);
    let mut chapter_infos: Vec<chapter::ChapterInfo> = Vec::new();
    let mut chapter = chapter::Chapter::new(config.get_title().clone());
    chapter.set_is_label_p(config.get_is_html_p());
    let mut book_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("index.html")?;
    chapter.write_title_w(&mut book_file)?;
    for line in bufreader.lines() {
        let line_str = line?;
        if blink_regex.is_match(&line_str) {
            continue;
        } else if chapter_regex.is_match(&line_str) {
            // chapter.flush(&mut book_file)?;
            writeln!(&mut book_file, "<mbp:pagebreak/>")?;
            chapter_infos.push(chapter.get_info());
            let order = chapter.get_current_order();
            chapter.restore_w(line_str, order + 1, &mut book_file)?;
        } else if is_use_subchapter && subchapter_regex.is_match(&line_str) {
            chapter.push_w(line_str, &mut book_file)?;
        } else {
            chapter.append_w(&line_str, &mut book_file)?;
        }
    }
    // chapter.flush(&mut book_file)?;
    chapter_infos.push(chapter.get_info());
    writeln!(&mut book_file, "\n</body>\n</html>")?;
    drop(book_file);
    {
        let file = File::create("toc.ncx")?;
        template_reg.render_to_write(&ncx_name, &chapter_infos, file)?;
    }
    {
        let file = File::create("toc.xhtml")?;
        template_reg.render_to_write(&toc_name, &chapter_infos, file)?;
    }
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("kindlegen.exe")
            .args(&vec![
                "-dont_append_source",
                "-c1",
                "-o",
                format!("{}", format_args!("{}.mobi", config.get_title())).as_str(),
                "book.opf",
            ])
            .output()
            .expect("failed to execute kindlegen")
    } else {
        std::process::Command::new("kindlegen")
            .args(&vec![
                "-dont_append_source",
                "-c1",
                "-o",
                format!("{}", format_args!("{}.mobi", config.get_title())).as_str(),
                "book.opf",
            ])
            .output()
            .expect("failed to execute kindlegen")
    };
    println!("{}", String::from_utf8_lossy(&output.stdout));
    std::fs::remove_file("index.html")?;
    std::fs::remove_file("book.opf")?;
    std::fs::remove_file("toc.ncx")?;
    std::fs::remove_file("toc.xhtml")?;
    Ok(())
}
