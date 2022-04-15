use dotenvy::dotenv;

use serde_json;
use std::any::Any;
use std::fs::{self};
use twitter_v2::authorization::{BearerToken, Oauth2Token};
use twitter_v2::data::ReferencedTweet;
use twitter_v2::data::ReferencedTweetKind::RepliedTo;
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

use async_recursion::async_recursion;

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
    //let conversation_id = "1513791040263651330";
    let conversation =
        get_twitter_conversation_from_tweet(get_tweet_by_id(1514707895992029195).await).await;
    println!("{:?}", &conversation);
    fs::write(
        "conversation.json",
        serde_json::to_string(&conversation).expect("Failed to parse tweets from API into JSON"),
    )
    .expect("Failed to write to conversation.json");
}

//work on this, it will need some recursion
//update... and it looks like you will need to look into async recursion
#[async_recursion]
async fn get_twitter_conversation_from_tweet(tweet: Tweet) -> Vec<Tweet> {
    let mut output = vec![tweet];
    match &output[0].referenced_tweets {
        Some(referenced_tweets) => {
            if referenced_tweets
                .iter()
                .any(|tweet| tweet.kind == RepliedTo)
            {
                //you don't want this to be index 0 but rather more precise
                let replied_to: Tweet = get_tweet_by_id(referenced_tweets[0].id.as_u64()).await;
                let mut conversation: Vec<Tweet> =
                    get_twitter_conversation_from_tweet(replied_to).await;
                output.append(&mut conversation);
                output
            } else {
                output
            }
        }
        None => output,
    }
}

//maybe generalised this to get tweets from query
async fn get_tweets_from_query(query: &str) -> Vec<Tweet> {
    load_api()
        .await
        .get_tweets_search_recent(query)
        .max_results(10)
        .tweet_fields([
            TweetField::Attachments,
            TweetField::ReferencedTweets,
            TweetField::ConversationId,
            TweetField::AuthorId,
            TweetField::CreatedAt,
        ])
        .send()
        .await
        .expect("Failed to get conversation")
        .into_data()
        .expect("Failed to open conversation Option<Vec<Tweet>>")
}

async fn get_tweets_from_user(user: &User) -> Vec<Tweet> {
    load_api()
        .await
        .get_user_tweets(user.id)
        .max_results(10) //this line gets the max results
        .tweet_fields([
            TweetField::Attachments,
            TweetField::ReferencedTweets,
            TweetField::ConversationId,
            TweetField::AuthorId,
            TweetField::CreatedAt,
        ])
        .send()
        .await
        .expect("Users tweets not loading")
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
