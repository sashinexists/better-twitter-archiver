use futures::stream::{self, StreamExt};
use ron;
use std::fs::{self};
use twitter_v2::authorization::BearerToken;
use twitter_v2::data::ReferencedTweetKind::RepliedTo;
use twitter_v2::query::{TweetField, UserField};
use twitter_v2::{Tweet, TwitterApi, User};

use async_recursion::async_recursion;

pub async fn load_tweet_from_id(id: u64) -> Tweet {
    match fs::read_to_string("data/tweets.ron") {
        Ok(_tweets_string) => {
            println!("Loading tweet from \"data/tweets.ron\"");
            let tweets: Vec<Tweet> = read_tweets_from_ron();
            match tweets.iter().find(|tweet| tweet.id == id) {
                Some(tweet) => tweet.clone(),
                None => {
                    let tweet = get_tweet_by_id(id).await;
                    let mut output: Vec<Tweet> = Vec::<Tweet>::new();
                    output.push(tweet.clone());
                    write_tweets_to_ron(&mut output);
                    tweet
                }
            }
        }
        Err(_error) => {
            println!("Loading tweet from API");
            let tweet = get_tweet_by_id(id).await;
            let mut tweets = Vec::<Tweet>::new();
            tweets.push(tweet.clone());
            write_tweets_to_ron(&mut tweets);
            tweet
        }
    }
}

pub async fn load_user_from_twitter_handle(twitter_handle: &str) -> User {
    match fs::read_to_string(format!("data/user-info_{twitter_handle}.ron")) {
        Ok(user) => {
            println!(
                "Loading user  @{twitter_handle} from \"data/user-info_{twitter_handle}.ron\""
            );
            ron::from_str(&user).expect(&format!(
                "Failed to parse file \"data/user_{twitter_handle}.ron\""
            ))
        }
        Err(_error) => {
            println!("Loading User from API");
            let user = get_user_by_twitter_handle(twitter_handle).await;
            fs::write(
                format!("data/user-info_{twitter_handle}.ron"),
                ron::ser::to_string_pretty(&user, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse user from API to JSON"),
            )
            .expect(&format!(
                "Failed to write file \"data/user-info_{twitter_handle}.ron\""
            ));
            user
        }
    }
}

pub async fn load_conversations_from_twitter_handle(twitter_handle: &str) -> Vec<Vec<Tweet>> {
    match fs::read_to_string(format!("data/user-conversations_{twitter_handle}.ron")) {
        Ok(conversations_string) => {
            println!("Loading conversations from \"data/user-conversations_{twitter_handle}.ron\"");
            ron::from_str(&conversations_string).expect(&format!(
                "Failed to parse conversations from \"data/user-conversations_{twitter_handle}"
            ))
        }
        Err(_error) => {
            println!("Loading conversations from API");
            let tweets = load_tweets_from_twitter_handle(twitter_handle).await;
            let conversations_stream = stream::iter(tweets);
            let conversations_then =
                conversations_stream.then(|tweet| get_twitter_conversation_from_tweet(tweet));
            let conversations = conversations_then.collect::<Vec<_>>().await;
            fs::write(
                format!("data/user-conversations_{twitter_handle}.ron"),
                ron::ser::to_string_pretty(&conversations, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse conversations from API into \"data/user-conversations_{twitter_handle}.ron\""),
            )
            .expect("Failed to write to \"data/user-conversations_{twitter_handle}.ron\"");
            conversations
        }
    }
}
//next up load conversations from data/conversations.ron
//if the last tweet in any conversation's id matched the input you can just  return that conversation
/*
pub async fn load_conversation_from_tweet_id(tweet_id: i64) -> Vec<Tweet> {
    match fs::read
}*/

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
                let replied_to: Tweet = load_tweet_from_id(replied_to_id).await;
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

async fn load_tweets_from_twitter_handle(twitter_handle: &str) -> Vec<Tweet> {
    match fs::read_to_string(format!("data/user-tweets_{twitter_handle}.ron")) {
        Ok(tweets) => ron::from_str(&tweets)
            .expect("Failed to load tweets from \"data/user-tweets_{twitter_handle}\""),
        Err(_error) => {
            let tweets =
                get_tweets_from_user(&load_user_from_twitter_handle(twitter_handle).await).await;
            fs::write(
                format!("data/user-tweets_{twitter_handle}.ron"),
                ron::ser::to_string_pretty(&tweets, ron::ser::PrettyConfig::new()).expect("Failed to parse fetched user tweets into \"data/user-tweets_{twitter_handle}.ron\"")
            ).expect("Failed to write @{twitter_handles}'s tweets into \"data/user-tweets_{}.ron\"");
            tweets
        }
    }
}

async fn get_tweet_by_id(id: u64) -> Tweet {
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

fn write_tweets_to_ron(tweets: &mut Vec<Tweet>) {
    match fs::read_to_string("data/tweets.ron") {
        Ok(tweets_from_ron_string) => {
            let mut tweets_from_ron: Vec<Tweet> = ron::from_str(&tweets_from_ron_string)
                .expect("Failed to read tweets from \"data/tweets.ron\"");
            tweets_from_ron.append(tweets);
            fs::write(
                "data/tweets.ron",
                ron::ser::to_string_pretty(&tweets_from_ron, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse tweets_from_ron back into a string"),
            )
            .expect("Failed to write to \"data/tweets.ron\"");
        }
        Err(_error) => fs::write(
            "data/tweets.ron",
            ron::ser::to_string_pretty(&tweets, ron::ser::PrettyConfig::new())
                .expect("Failed to parse tweets into a ron string"),
        )
        .expect("Failed to create a new \"data/tweets.ron\""),
    }
}

fn read_tweets_from_ron() -> Vec<Tweet> {
    match fs::read_to_string("data/tweets.ron") {
        Ok(tweets) => {
            ron::from_str(&tweets).expect("Failed to parse tweets from \"data/tweets.ron\"")
        }
        Err(_error) => Vec::<Tweet>::new(),
    }
}
