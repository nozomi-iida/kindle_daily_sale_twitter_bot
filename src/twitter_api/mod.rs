use egg_mode::tweet::DraftTweet;
use crate::common;

pub async fn create_tweet(content: String) -> Result<(), Box<dyn std::error::Error>> {
  let config = common::Config::load().await;
  let tweet = DraftTweet::new(content.clone());
  tweet.send(&config.token).await?;
  Ok(())
}
