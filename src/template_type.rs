#[derive(Debug)]
pub enum TemplateType {
    Opf,
    Catalog,
    Content,
    Ncx,
    Cover,
    Title,
}

impl std::fmt::Display for TemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl TemplateType {
    pub const VALUES: [Self; 6] = [
        Self::Opf,
        Self::Catalog,
        Self::Content,
        Self::Ncx,
        Self::Cover,
        Self::Title,
    ];
    pub fn get_file_name(&self) -> &'static str {
        match *self {
            Self::Opf => "content.opf",
            Self::Catalog => "catalog.xhtml",
            Self::Content => "content.xhtml",
            Self::Ncx => "toc.ncx",
            Self::Cover => "cover.xhtml",
            Self::Title => "title.xhtml",
        }
    }
}
