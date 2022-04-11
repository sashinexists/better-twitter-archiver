use dotenvy::dotenv;

use serde_json;
use std::fs::{self};
use twitter_v2::authorization::BearerToken;
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

const TWITTER_HANDLE: &str = "yudapearl";
#[tokio::main]
async fn main() {
    dotenv().ok();
    let user = get_user_by_twitter_handle(TWITTER_HANDLE).await;
    let tweets: Vec<Tweet> = match fs::read_to_string("tweets.json") {
        Ok(tweets) => {
            println!(
                "Successfully read tweets.json. \n\nContent is as follows:\n\n{}",
                &tweets
            );
            serde_json::from_str(&tweets).expect("Failed to parse file tweets.json")
        }
        Err(_error) => {
            println!("Loading tweets from API...");
            let tweets: Vec<Tweet> = get_tweets_from_user(&user).await;
            fs::write(
                "tweets.json",
                serde_json::to_string(&tweets).expect("Failed to parse tweets from API into JSON"),
            )
            .expect("Failed to write to tweets.json");
            println!("Saved tweets to new file tweets.json");
            tweets
        }
    };
}

async fn get_tweets_from_user(user: &User) -> Vec<Tweet> {
    load_api()
        .await
        .get_user_tweets(user.id)
        .max_results(20) //this line gets the max results
        .tweet_fields([
            TweetField::Attachments,
            TweetField::ReferencedTweets,
            TweetField::ConversationId,
            TweetField::AuthorId,
            TweetField::CreatedAt,
        ])
        .send()
        .await
        .expect("Users tweets net loading")
        .into_data()
        .expect("Failure to open option<Vec<Tweet>>")
}

#[allow(dead_code)]
async fn get_tweet_by_id(id: u64) -> Tweet {
    load_api()
        .await
        .get_tweet(id)
        .tweet_fields([
            TweetField::Attachments,
            TweetField::ContextAnnotations,
            TweetField::ReferencedTweets,
            TweetField::ConversationId,
            TweetField::AuthorId,
            TweetField::CreatedAt,
        ])
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
