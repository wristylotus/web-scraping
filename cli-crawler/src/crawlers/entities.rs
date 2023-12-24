use reqwest::Url;

pub struct Page {
    pub link: Url,
    pub content: String,
}

impl Page {
    pub fn new(link: Url, content: String) -> Self {
        Self { link, content }
    }
}