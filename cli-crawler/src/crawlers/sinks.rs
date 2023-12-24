use std::cell::RefCell;
use std::cmp::min;
use redis::Commands;
use anyhow::{Ok, Result};
use crate::crawlers::entities::Page;

pub trait Sink {
    type Message;

    fn write(&mut self, msg: Self::Message) -> Result<()>;
}

pub struct OutputPrinter();

impl Sink for OutputPrinter {
    type Message = Page;

    fn write(&mut self, msg: Self::Message) -> Result<()> {
        println!("Link: {}", msg.link);
        println!(
            "Content: {} ...",
            msg.content[0..min(200, msg.content.len())].replace('\n', "")
        );

        Ok(())
    }
}

pub struct RedisSink {
    redis_conn: RefCell<redis::Connection>,
    namespace: String,
}

impl RedisSink {
    pub fn new(redis_url: &str, namespace: &str) -> Self {
        let client = redis::Client::open(redis_url).expect("Cannot open redis connection");
        let redis_conn = client.get_connection().expect("Cannot get redis connection");

        Self { redis_conn: RefCell::new(redis_conn), namespace: namespace.to_string() }
    }

    fn namespace_key(&self, key: &str) -> String {
        format!("{}:{}", self.namespace, key)
    }
}

impl Sink for RedisSink {
    type Message = Page;

    fn write(&mut self, msg: Self::Message) -> Result<()> {
        let domain = msg.link.domain().expect("Cannot extract domain name!");
        let items = Vec::from([
            ("link", msg.link.to_string()),
            ("content", msg.content)
        ]);

        self.redis_conn.borrow_mut().xadd(self.namespace_key(&domain), "*", &items)?;

        Ok(())
    }
}