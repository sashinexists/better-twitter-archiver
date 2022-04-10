use dotenvy::dotenv;
use futures::executor::block_on;

use core::fmt::Debug;

use twitter_v2::authorization::BearerToken;
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let user = block_on(get_user_by_handle("yudapearl"));
    let tweet = block_on(get_tweet_by_id(1012187366587392000));
    print_twitter_data(user).await;
    print_twitter_data(tweet).await;
}

async fn print_twitter_data<T: Debug>(twitter_data: Option<T>) {
    match twitter_data {
        Some(twitter_data) => println!("{:?}\n", twitter_data),
        None => println!("Twitter data not found"),
    }
}

async fn get_tweet_by_id(id: u64) -> Option<Tweet> {
    load_api()
        .await
        .get_tweet(id)
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .send()
        .await
        .expect("this tweet should exist")
        .into_data()
}

async fn get_user_by_handle(id: &str) -> Option<User> {
    load_api()
        .await
        .get_user_by_username(id)
        .user_fields([UserField::Username, UserField::Description])
        .send()
        .await
        .expect("This user should exist")
        .into_data()
}

async fn load_api() -> TwitterApi<BearerToken> {
    let auth = BearerToken::new(std::env::var("TWITTER_DEV_BEARER_TOKEN").unwrap());
    TwitterApi::new(auth)
}
