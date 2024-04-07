use async_std::task;
use crawler::crawl::box_crawl;
use url::Url;


fn main() {
    println!("Hello, world!");
    task::block_on(async {
        // box_crawl(vec![Url::parse("https://www.rust-lang.org").unwrap()], 1, 2).await
        box_crawl(vec![Url::parse("https://www.apple.com").unwrap()], 1, 2).await
    });
}
