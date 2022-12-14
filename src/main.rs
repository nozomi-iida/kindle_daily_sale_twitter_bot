use chrono::Local;
use dotenv::dotenv;

mod twitter_api;
mod amazon_crawler;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let books = amazon_crawler::get_daily_sale_books();
  let date = Local::now().format("%Y年%m月").to_string();
  for book in books {
    let content = format!("【{}のKindle日替わりセール】\n {}\n {}", date, book.title, book.amazon_url);
      let _ = twitter_api::create_tweet(content).await;
  }
  // let _ = twitter_api::delete_all_tweet().await;
}
