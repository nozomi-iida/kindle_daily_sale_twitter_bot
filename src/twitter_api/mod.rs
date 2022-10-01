mod auth;

use egg_mode::tweet;
use egg_mode::tweet::DraftTweet;

pub async fn create_tweet(content: String) -> Result<(), Box<dyn std::error::Error>> {
  let config = auth::Config::load().await;
  let tweet = DraftTweet::new(content.clone());
  tweet.send(&config.token).await?;
  Ok(())
}

pub async fn delete_all_tweet() {
  let config = auth::Config::load().await;
  let tweets = tweet::user_timeline(config.user_id, true, false, &config.token).with_page_size(200);
  let (_home, feed) = tweets.start().await.unwrap();
  for status in feed.into_iter() {
    let _ = tweet::delete(status.response.id, &config.token).await.unwrap();
  }
}
