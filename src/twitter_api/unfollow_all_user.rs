mod auth;

use dotenv::dotenv;
use futures::TryStreamExt;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let config = auth::Config::load().await;

  let mut follower_ids = Vec::new();
  egg_mode::user::friends_ids(config.user_id, &config.token).try_for_each( |resp| {
    follower_ids.push(resp.response);
    futures::future::ok(())
  }).await.unwrap();

  for id in follower_ids.into_iter() {
    let _ = egg_mode::user::unfollow(id, &config.token).await.unwrap();
  }
}
