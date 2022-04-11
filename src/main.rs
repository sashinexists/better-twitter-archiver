use dotenvy::dotenv;
use futures::executor::block_on;

use core::fmt::Debug;

use serde_json;
use std::fs::{self};
use twitter_v2::authorization::BearerToken;
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let user = get_user_by_twitter_handle("yudapearl").await;
    let tweets = get_tweets_from_user(&user).await;
    let tweets_json: String = serde_json::to_string(&tweets).expect("Failed to deserialise tweets");
    fs::write("tweets.json", tweets_json).expect("Failed to write to tweets.json");
}

async fn get_tweets_from_user(user: &User) -> Vec<Tweet> {
    load_api()
        .await
        .get_user_tweets(user.id)
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .send()
        .await
        .expect("Users tweets net loading")
        .into_data()
        .expect("Failure to open option<Vec<Tweet>>")
}

async fn get_tweet_by_id(id: u64) -> Tweet {
    load_api()
        .await
        .get_tweet(id)
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .send()
        .await
        .expect("this tweet should exist")
        .into_data()
        .expect("Failure to open option<Tweet>")
}

async fn get_user_by_twitter_handle(twitter_handle: &str) -> User {
    load_api()
        .await
        .get_user_by_username(twitter_handle)
        .user_fields([UserField::Username, UserField::Description])
        .send()
        .await
        .expect("This user should exist")
        .into_data()
        .expect("Failure to open Option<User>")
}

async fn load_api() -> TwitterApi<BearerToken> {
    let auth = BearerToken::new(std::env::var("TWITTER_DEV_BEARER_TOKEN").unwrap());
    TwitterApi::new(auth)
}
