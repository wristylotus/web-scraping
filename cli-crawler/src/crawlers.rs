pub mod sinks;
pub mod states;
mod entities;

use crate::tools::url::UrlRoot;
use anyhow::{Ok, Result};
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};

use sinks::Sink;
use states::State;
use entities::Page;

pub struct Crawler {
    state: Box<dyn State>,
    sink: Box<dyn Sink<Message=Page>>,
    links_selector: Selector,
}

impl Crawler {
    pub fn new(state: Box<dyn State>, sink: Box<dyn Sink<Message=Page>>) -> Self {
        Self { state, sink, links_selector: Selector::parse("a").unwrap() }
    }

    pub fn run(&mut self) -> Result<()> {
        let file_pattern = Regex::new(r"\.\w+$")?;

        while let Some(page_link) = self.state.next_page() {
            let page_link: Url = page_link.parse()?;
            let resp: String = reqwest::blocking::get(page_link.clone())?.text()?;
            let page_content = Html::parse_document(&resp);

            self.sink.write(Page::new(page_link.clone(), page_content.html()))?;

            self.state.add_page_to_visited(&page_link.to_string())?;

            let non_visited_links: Vec<String> = self
                .find_all_links(&page_link, &page_content)
                .into_iter()
                .filter(|link| !file_pattern.is_match(link))
                .filter(|link| self.state.non_page_visited(link))
                .collect();

            self.state.add_pages_to_visit(non_visited_links)?;
        }

        Ok(())
    }

    fn find_all_links(&mut self, page_link: &Url, page_content: &Html) -> Vec<String> {
        page_content
            .select(&self.links_selector)
            .flat_map(|element| element.attr("href"))
            .flat_map(|link| Crawler::resolve_link(&page_link, link))
            .collect()
    }

    fn resolve_link(host_link: &Url, link: &str) -> Option<String> {
        let host_link = host_link.root()?;

        if link.starts_with("/") {
            Some(format!("{}{}", host_link, link))
        } else if link.starts_with(&host_link) {
            Some(link.to_string())
        } else {
            None
        }
    }
}
