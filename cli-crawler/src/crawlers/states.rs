use std::{
    cell::RefCell,
    collections::{BTreeSet, HashSet},
};

use anyhow::{Ok, Result};
use redis::Commands;

pub trait State {
    fn add_page_to_visit(&mut self, link: &str) -> Result<()>;

    fn add_pages_to_visit(&mut self, links: Vec<String>) -> Result<()> {
        links.into_iter().for_each(|link| {
            self.add_page_to_visit(&link).unwrap();
        });

        Ok(())
    }

    fn add_page_to_visited(&mut self, link: &str) -> Result<()>;

    fn is_page_visited(&self, link: &str) -> bool;

    fn non_page_visited(&self, link: &str) -> bool {
        !self.is_page_visited(link)
    }

    fn next_page(&mut self) -> Option<String>;

    fn reset(&mut self) -> Result<()>;
}

pub struct LocalState {
    pages_to_visit: BTreeSet<String>,
    visited_pages: HashSet<String>,
}

impl LocalState {
    pub fn new(root_page: &str) -> Self {
        Self {
            pages_to_visit: BTreeSet::from([root_page.to_string()]),
            visited_pages: HashSet::new(),
        }
    }
}

impl State for LocalState {
    fn add_page_to_visit(&mut self, link: &str) -> Result<()> {
        self.pages_to_visit.insert(link.to_string());

        Ok(())
    }

    fn add_page_to_visited(&mut self, link: &str) -> Result<()> {
        self.visited_pages.insert(link.to_string());

        Ok(())
    }

    fn is_page_visited(&self, link: &str) -> bool {
        self.visited_pages.contains(link)
    }

    fn next_page(&mut self) -> Option<String> {
        self.pages_to_visit.pop_first()
    }

    fn reset(&mut self) -> Result<()> {
        self.pages_to_visit.clear();
        self.visited_pages.clear();

        Ok(())
    }
}

pub struct RedisState {
    redis_conn: RefCell<redis::Connection>,
    namespace: String,
}

impl RedisState {
    pub fn new(redis_url: &str, namespace: &str) -> Self {
        let client = redis::Client::open(redis_url).expect("Cannot open redis connection");
        let redis_conn = client.get_connection().expect("Cannot get redis connection");

        Self { redis_conn: RefCell::new(redis_conn), namespace: namespace.to_string() }
    }

    fn namespace_key(&self, key: &str) -> String {
        format!("{}:{}", self.namespace, key)
    }
}

impl State for RedisState {
    fn add_page_to_visit(&mut self, link: &str) -> Result<()> {
        self.redis_conn.borrow_mut().sadd(self.namespace_key("pages-to-visit"), link)?;

        Ok(())
    }

    fn add_page_to_visited(&mut self, link: &str) -> Result<()> {
        self.redis_conn.borrow_mut().sadd(self.namespace_key("visited-pages"), link)?;

        Ok(())
    }

    fn is_page_visited(&self, link: &str) -> bool {
        let result: i32 = self.redis_conn.borrow_mut().sismember(self.namespace_key("visited-pages"), link).unwrap();

        result == 1
    }

    fn next_page(&mut self) -> Option<String> {
        let page_link: Option<String> =
            self.redis_conn.borrow_mut().spop(self.namespace_key("pages-to-visit")).unwrap();

        page_link
    }

    fn reset(&mut self) -> Result<()> {
        self.redis_conn.borrow_mut().del(self.namespace_key("pages-to-visit"))?;
        self.redis_conn.borrow_mut().del(self.namespace_key("visited-pages"))?;

        Ok(())
    }
}
