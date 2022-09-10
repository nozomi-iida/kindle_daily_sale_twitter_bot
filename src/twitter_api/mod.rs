use std::io::{stdout, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use chrono::format::Item::Error;
use egg_mode::media::{get_status, media_types, ProgressInfo, set_metadata, upload_media};
use egg_mode::tweet::DraftTweet;
use structopt::StructOpt;
use crate::common;

#[derive(StructOpt)]
/// A simple CLI for uploading a tweet, optionally with media attached
struct Args {
  /// Text of the tweet
  text: String,
}

pub async fn create_tweet() -> Result<(), Box<dyn std::error::Error>> {
  let args: Args = Args::from_args();
  let config = common::Config::load().await;
  let tweet = DraftTweet::new(args.text.clone());
  tweet.send(&config.token).await?;
  println!("Sent tweet: '{}'", args.text);
  Ok(())
}
