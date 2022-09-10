use dotenv::dotenv;

mod common;
mod twitter_api;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let _ = twitter_api::create_tweet().await;
}
