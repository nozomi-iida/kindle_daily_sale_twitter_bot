use std::io::{stdout, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use chrono::format::Item::Error;
use egg_mode::media::{get_status, media_types, ProgressInfo, set_metadata, upload_media};
use egg_mode::tweet::DraftTweet;
use structopt::StructOpt;
use crate::common;

pub async fn create_tweet(content: String) -> Result<(), Box<dyn std::error::Error>> {
  let config = common::Config::load().await;
  let tweet = DraftTweet::new(content.clone());
  tweet.send(&config.token).await?;
  Ok(())
}
