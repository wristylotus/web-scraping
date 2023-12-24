use anyhow::Result;

use cli_crawler::sinks::RedisSink;
use cli_crawler::states::{RedisState, State};
use cli_crawler::Crawler;

fn main() -> Result<()> {
    let root_page = "https://www.rust-lang.org/";
    let redis_url = "redis://127.0.0.1:6379/";
    let redis_namespace = "crawler";

    let mut state = Box::new(RedisState::new(redis_url, redis_namespace));
    let sink = Box::new(RedisSink::new(redis_url, redis_namespace));

    state.reset()?;
    state.add_page_to_visit(root_page)?;

    let mut crawler = Crawler::new(state, sink);

    crawler.run()?;

    Ok(())
}
