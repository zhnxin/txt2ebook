#[warn(dead_code)]
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ChapterContent {
    is_not_sub: bool,
    title: String,
    content: String,
    key: String,
    order: isize,
}
#[derive(Serialize, Debug)]
pub struct ChapterMeta {
    title: String,
    key: String,
    order: isize,
}
#[derive(Serialize, Debug)]
pub struct ChapterInfo {
    chapter: ChapterMeta,
    sub: Vec<ChapterMeta>,
}

impl ChapterContent {
    fn append(&mut self, content: &String, is_label_p: bool) {
        if is_label_p {
            self.content.push_str("<p>");
            self.content.push_str(content.trim());
            self.content.push_str("</p>\n");
        } else {
            self.content.push_str(content);
            self.content.push('\n');
        }
    }
    fn set_title(&mut self, title: String) {
        self.title = title;
    }
    fn write_title_w(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
        if self.is_not_sub {
            write!(
                w,
                "<a name=\"{}\"/><h2 class=\"chapter\" id=\"{}\">{}</h2>\n",
                &self.key,
                &self.key,
                htmlescape::encode_minimal(&self.title)
            )
        } else {
            write!(
                w,
                "<a name=\"{}\"/><h2 class=\"subchapter\" id=\"{}\">{}</h2>\n",
                &self.key,
                &self.key,
                htmlescape::encode_minimal(&self.title)
            )
        }
    }
    fn append_w(
        &self,
        content: &String,
        is_label_p: bool,
        mut w: impl std::io::Write,
    ) -> std::io::Result<()> {
        if is_label_p {
            writeln!(
                w,
                "<p>{}</p><br/>",
                htmlescape::encode_minimal(content.trim())
            )
        } else {
            writeln!(w, "{}<br/>", htmlescape::encode_minimal(content.trim()))
        }
    }
    fn restore_w(
        &mut self,
        title: String,
        key: String,
        order: isize,
        mut w: impl std::io::Write,
    ) -> std::io::Result<()> {
        self.title = title;
        self.key = key;
        self.order = order;
        self.content.clear();
        if self.is_not_sub {
            write!(
                w,
                "<a name=\"{}\"/><h2 class=\"chapter\" id=\"{}\">{}</h2>\n",
                &self.key,
                &self.key,
                htmlescape::encode_minimal(&self.title)
            )
        } else {
            write!(
                w,
                "<a name=\"{}\"/><h2 class=\"subchapter\" id=\"{}\">{}</h2>\n",
                &self.key,
                &self.key,
                htmlescape::encode_minimal(&self.title)
            )
        }
    }
    fn restore(&mut self, title: String, key: String, order: isize) {
        self.title = title;
        self.key = key;
        self.order = order;
        self.content.clear();
    }
    fn get_info(&self) -> ChapterMeta {
        ChapterMeta {
            title: self.title.clone(),
            key: self.key.clone(),
            order: self.order,
        }
    }
    fn to_html(&self) -> String {
        let mut content = String::new();
        if self.is_not_sub {
            content.push_str(&format!(
                "{:?}",
                format_args!(
                    "<a name=\"{}\"/><h2 class=\"chapter\" id=\"{}\">{}</h2>\n",
                    &self.key, &self.key, &self.title
                )
            ));
        } else {
            content.push_str(&format!(
                "{:?}",
                format_args!(
                    "<a name=\"{}\"/><h2 class=\"subchapter\" id=\"{}\">{}</h2>\n",
                    &self.key, &self.key, &self.title
                )
            ));
        }
        content.push_str(&self.content);
        content.push_str("<mbp:pagebreak/>\n");
        content
    }
}

pub struct Chapter {
    content: ChapterContent,
    current_order: isize,
    sub_chapter: Vec<ChapterContent>,
    sub_length: usize,
    is_lable_p: bool,
}

impl Chapter {
    pub fn new(title: String) -> Chapter {
        Chapter {
            content: ChapterContent {
                title: title,
                is_not_sub: true,
                order: 1,
                key: String::from("chap1"),
                content: String::new(),
            },
            sub_chapter: Vec::new(),
            current_order: 1,
            sub_length: 0,
            is_lable_p: false,
        }
    }
    pub fn write_title_w(&self,w:impl std::io::Write)-> std::io::Result<()>{
        self.content.write_title_w(w)
    }
    pub fn get_current_order(&self) -> isize {
        self.current_order
    }
    pub fn set_is_label_p(&mut self, flag: bool) {
        self.is_lable_p = flag;
    }

    pub fn set_order(&mut self, i: isize) {
        self.current_order = i;
    }
    pub fn restore(&mut self, title: String, order: isize) {
        self.sub_length = 0;
        self.content.title = title;
        self.content.content.clear();
        self.content.key = format!("{:?}", format_args!("chap{}", order));
        self.sub_chapter.clear();
        self.current_order = order;
    }
    pub fn restore_w(
        &mut self,
        title: String,
        order: isize,
        w: impl std::io::Write,
    ) -> std::io::Result<()> {
        self.sub_length = 0;
        self.content.title = title;
        self.content.content.clear();
        self.content.key = format!("{:?}", format_args!("chap{}", order));
        self.sub_chapter.clear();
        self.current_order = order;
        self.content.write_title_w(w)
    }
    pub fn push(&mut self, title: String) {
        self.current_order += 1;
        self.sub_length += 1;
        self.sub_chapter.push(ChapterContent {
            title: title,
            content: String::new(),
            order: self.current_order,
            is_not_sub: false,
            key: format!("{:?}", format_args!("chap{}", self.current_order)),
        });
    }

    pub fn push_w(&mut self, title: String, mut w: impl std::io::Write) -> std::io::Result<()> {
        self.current_order += 1;
        if self.sub_length > 0 {
            writeln!(w, "<mbp:pagebreak/>")?;
        }
        self.sub_length += 1;
        let chap = ChapterContent {
            title: title,
            content: String::new(),
            order: self.current_order,
            is_not_sub: false,
            key: format!("{:?}", format_args!("chap{}", self.current_order)),
        };
        chap.write_title_w(w)?;
        self.sub_chapter.push(chap);
        Ok(())
    }

    pub fn get_info(&self) -> ChapterInfo {
        let mut info: ChapterInfo = ChapterInfo {
            chapter: self.content.get_info(),
            sub: Vec::with_capacity(self.sub_length),
        };
        if !self.sub_chapter.is_empty() {
            for chapter in self.sub_chapter.iter() {
                info.sub.push(chapter.get_info());
            }
        }
        info
    }
    pub fn append(&mut self, content: &String) {
        if self.sub_length < 1 {
            self.content.append(content, self.is_lable_p);
        } else {
            self.sub_chapter[self.sub_length - 1].append(content, self.is_lable_p)
        }
    }
    pub fn append_w(&mut self, content: &String, w: impl std::io::Write) -> std::io::Result<()> {
        if self.sub_length < 1 {
            self.content.append_w(content, self.is_lable_p, w)
        } else {
            self.sub_chapter[self.sub_length - 1].append_w(content, self.is_lable_p, w)
        }
    }
    pub fn flush(&mut self, mut w: impl std::io::Write) -> std::io::Result<()> {
        w.write(self.content.to_html().as_bytes())?;
        for ch in self.sub_chapter.iter() {
            w.write(ch.to_html().as_bytes())?;
        }
        w.flush()?;
        Ok(())
    }
}
