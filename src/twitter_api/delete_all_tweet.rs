mod auth;

use egg_mode::tweet;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let config = auth::Config::load().await;
  let tweets = tweet::user_timeline(config.user_id, true, false, &config.token).with_page_size(200);
  let (_home, feed) = tweets.start().await.unwrap();
  for status in feed.into_iter() {
    let _ = tweet::delete(status.response.id, &config.token).await.unwrap();
  }
}
