#[warn(dead_code)]
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ChapterMeta {
    title: String,
    is_sub_cha: bool,
    has_sub_chap: bool,
    id: usize,
}

impl ChapterMeta{
    pub fn new(title:String,id:usize) ->Self{
        Self{
            title:title,
            is_sub_cha:false,
            has_sub_chap:false,
            id:id,
        }
    }

    pub fn new_sub(title:String,id:usize) -> Self{
        Self{
            title:title,
            is_sub_cha:true,
            has_sub_chap:false,
            id:id,
        }
    }

    pub fn set_title(&mut self,title:String){
        self.title = title;
    }
    pub fn set_id(&mut self,id:usize){
        self.id = id;
    }

    pub fn set_has_sub_chap(&mut self,is_has:bool){
        self.has_sub_chap = is_has;
    }
}

#[derive(Serialize, Debug)]
pub struct ChapterInfo {
    chapter: ChapterMeta,
    subchapter: Vec<ChapterMeta>,
}

impl ChapterInfo {
    pub fn new(title: String, id: usize) -> Self {
        Self {
            chapter: ChapterMeta::new(title, id),
            subchapter: Vec::new(),
        }
    }

    pub fn add_subchapter(&mut self, title: String, id: usize) {
        self.chapter.has_sub_chap = true;
        self.subchapter.push(ChapterMeta::new_sub(title, id))
    }
}
#[derive(Serialize, Debug)]
pub struct ChapterContent {
    metadata:ChapterMeta,
    content: String,
}
impl ChapterContent {
    pub fn new(title: String,id:usize) -> Self {
        Self {
            metadata:ChapterMeta::new(title,id),
            content: String::new(),
        }
    }
    pub fn new_subchapter(title: String,id:usize) ->Self{
        Self{
            metadata:ChapterMeta::new_sub(title,id),
            content:String::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    pub fn get_id(&self) ->usize{
        self.metadata.id
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
        self.metadata.set_title(title);
    }
    pub fn restore(&mut self, title: String,id:usize) {
        self.set_title(title);
        self.set_has_sub_chap(false);
        self.metadata.id = id;
        self.content.clear();
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    pub fn get_is_sub_cha(&self) -> bool {
        self.metadata.is_sub_cha
    }
    pub fn set_has_sub_chap(&mut self,is_has:bool){
        self.metadata.set_has_sub_chap(is_has);
    }
}

#[derive(Serialize, Debug)]
pub struct MainChapter{
    data: ChapterContent,
    subchapter:Vec<ChapterContent>,
}

impl MainChapter {
    pub fn new(title:String,id:usize) ->Self{
        Self{
            data:ChapterContent::new(title,id),
            subchapter:Vec::new(),
        }
    }

    pub fn is_empty(&self)->bool{
        self.data.is_empty() && self.subchapter.is_empty()
    }

    pub fn get_id(&self) ->usize{
        self.data.get_id()
    }

    pub fn restore(&mut self,title:String,id:usize){
        self.data.restore(title,id);
        self.subchapter.clear();
    }

    pub fn add_subchapter(&mut self,title:String,id:usize){
        self.data.set_has_sub_chap(true);
        self.subchapter.push(ChapterContent::new_subchapter(title,id));
    }

    pub fn append(&mut self,content:&String,is_label_p:bool){
        if let Some(c) = self.subchapter.last_mut(){
            c.append(content, is_label_p);
        }else{
            self.data.append(content, is_label_p);
        }
    }

    pub fn get_chapter_content(&self)->String{
        self.data.get_content()
    }
}