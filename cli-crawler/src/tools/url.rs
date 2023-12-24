use reqwest::Url;

pub trait UrlRoot {
    fn root(&self) -> Option<String>;
}

impl UrlRoot for Url {
    fn root(&self) -> Option<String> {
        Some(format!("{}://{}", self.scheme(), self.domain()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_root_extraction() {
        let url = Url::parse("https://www.rust-lang.org/test/cats").unwrap();

        assert_eq!(url.root().unwrap(), "https://www.rust-lang.org");
    }
}
