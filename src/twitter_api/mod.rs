mod auth;

use egg_mode::tweet;
use egg_mode::tweet::DraftTweet;

pub async fn create_tweet(content: String) -> Result<(), Box<dyn std::error::Error>> {
  let config = auth::Config::load().await;
  let tweet = DraftTweet::new(content.clone());
  tweet.send(&config.token).await?;
  Ok(())
}
