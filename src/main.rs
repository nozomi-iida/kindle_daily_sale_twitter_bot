use chrono::Local;
use dotenv::dotenv;

mod common;
mod twitter_api;
mod amazon_crawler;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let books = amazon_crawler::get_daily_sale_books();
  let date = Local::now().format("%Y年%m月").to_string();
  for book in books {
    let content = format!("{}のKindle日替わりセール\n {}: {}", date, book.title, book.amazon_url);
    println!("{:?}", content);
    //   let _ = twitter_api::create_tweet(content).await;
  }
}
