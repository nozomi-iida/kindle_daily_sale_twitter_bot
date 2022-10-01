use std;
use std::env;

pub struct Config {
  pub token: egg_mode::Token,
  pub user_id: u64,
}

impl Config {
  pub async fn load() -> Self {
    let a1 = Config::load_inner().await;
    if let Some(conf) = a1 {
      return conf;
    }

    Config::load_inner().await.unwrap()
  }

  // tokenからユーザー情報を取得する
  pub async fn load_inner() -> Option<Self> {
    let consumer_key = env::var("TWITTER_API_KEY").unwrap();
    let consumer_secret = env::var("TWITTER_API_SECRET").unwrap();
    let access_key = env::var("TWITTER_ACCESS_KEY").unwrap();
    let access_secret = env::var("TWITTER_ACCESS_SECRET").unwrap();
    let con_token = egg_mode::KeyPair::new(
      consumer_key,
      consumer_secret
    );
    let access_token = egg_mode::KeyPair::new(
      access_key,
      access_secret
    );
    let token = egg_mode::Token::Access{
      consumer: con_token,
      access: access_token
    };
    let user = egg_mode::auth::verify_tokens(&token).await.unwrap().response;

    return Some(Config {
      token,
      user_id: user.id
    })
  }
}