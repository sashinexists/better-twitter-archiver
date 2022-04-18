use futures::stream::{self, StreamExt};
use ron;
use std::fs::{self};
use twitter_v2::authorization::BearerToken;
use twitter_v2::data::ReferencedTweetKind::RepliedTo;
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

use async_recursion::async_recursion;

const TWITTER_HANDLE: &str = "yudapearl";
#[tokio::main]
pub async fn main() {
    //dotenv().ok();
    let user = load_user().await;
    let tweets: Vec<Tweet> = load_tweets(&user).await;
    let conversations = load_conversations(tweets).await;
}

async fn load_user() -> User {
    match fs::read_to_string("user.ron") {
        Ok(user) => {
            println!("Successfully read user.ron");
            ron::from_str(&user).expect("Failed to parse file user.ron")
        }
        Err(_error) => {
            println!("Loading User from API");
            let user = get_user_by_twitter_handle(TWITTER_HANDLE).await;
            fs::write(
                "user.ron",
                ron::to_string(&user).expect("Failed to parse user from API to JSON"),
            )
            .expect("Failed to write file user.ron");
            user
        }
    }
}

async fn load_tweets(user: &User) -> Vec<Tweet> {
    match fs::read_to_string("tweets.ron") {
        Ok(tweets) => {
            println!("Successfully read tweets.ron.");
            ron::from_str(&tweets).expect("Failed to parse file tweets.ron")
        }
        Err(_error) => {
            println!("Loading tweets from API...");
            let tweets: Vec<Tweet> = get_tweets_from_user(&user).await;
            fs::write(
                "tweets.ron",
                ron::to_string(&tweets).expect("Failed to parse tweets from API into JSON"),
            )
            .expect("Failed to write to tweets.ron");
            println!("Saved tweets to new file tweets.ron");
            tweets
        }
    }
}

pub async fn load_conversations(tweets: Vec<Tweet>) -> Vec<Vec<Tweet>> {
    match fs::read_to_string("conversations.ron") {
        Ok(conversations) => {
            println!("Successully read conversations.ron");
            ron::from_str(&conversations).expect("Failed to parse file conversations.ron;")
        }
        Err(_error) => {
            println!("Loading conversations from API");
            let conversations_stream = stream::iter(tweets);
            let conversations_then =
                conversations_stream.then(|tweet| get_twitter_conversation_from_tweet(tweet));
            let conversations = conversations_then.collect::<Vec<_>>().await;

            fs::write(
                "conversations.ron",
                ron::to_string(&conversations)
                    .expect("Failed to parse conversations from API into a JSON file"),
            )
            .expect("Failed to write to conversations.ron");
            conversations
        }
    }
}

#[async_recursion]
pub async fn get_twitter_conversation_from_tweet(tweet: Tweet) -> Vec<Tweet> {
    let mut output = vec![tweet];
    match &output[0].referenced_tweets {
        Some(referenced_tweets) => {
            if referenced_tweets
                .iter()
                .any(|tweet| tweet.kind == RepliedTo)
            {
                let replied_to_id = referenced_tweets
                    .iter()
                    .find(|tweet| tweet.kind == RepliedTo)
                    .expect("Failed to find replied to tweet")
                    .id
                    .as_u64();
                let replied_to: Tweet = get_tweet_by_id(replied_to_id).await;
                let mut conversation: Vec<Tweet> =
                    get_twitter_conversation_from_tweet(replied_to).await;
                output.append(&mut conversation);
                output
            } else {
                output.reverse();
                output
            }
        }
        None => output,
    }
}

#[allow(dead_code)]
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

pub async fn get_tweets_from_user(user: &User) -> Vec<Tweet> {
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

pub async fn get_tweet_by_id(id: u64) -> Tweet {
    load_api()
        .await
        .get_tweet(id)
        .tweet_fields([
            TweetField::Attachments,
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

pub async fn get_user_by_twitter_handle(twitter_handle: &str) -> User {
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
