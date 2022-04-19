use std::fs;

use futures::stream::{self, StreamExt};
use ron;

use twitter_v2::{Tweet, User};

pub mod api;
pub mod io;

pub async fn load_tweet_from_id(id: u64) -> Tweet {
    match fs::read_to_string("data/tweets.ron") {
        Ok(_tweets_string) => {
            println!("Loading tweet from \"data/tweets.ron\"");
            let tweets: Vec<Tweet> = io::read_tweets_from_ron();
            match tweets.iter().find(|tweet| tweet.id == id) {
                Some(tweet) => tweet.clone(),
                None => {
                    let tweet = api::get_tweet_by_id(id).await;
                    let mut output: Vec<Tweet> = Vec::<Tweet>::new();
                    output.push(tweet.clone());
                    io::write_tweets_to_ron(&mut output);
                    tweet
                }
            }
        }
        Err(_error) => {
            println!("Loading tweet from API");
            let tweet = api::get_tweet_by_id(id).await;
            let mut tweets = Vec::<Tweet>::new();
            tweets.push(tweet.clone());
            io::write_tweets_to_ron(&mut tweets);
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
            let user = api::get_user_by_twitter_handle(twitter_handle).await;
            io::write_user_info_to_ron(&user, twitter_handle);
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
            let conversations_then = conversations_stream
                .then(|tweet| load_conversation_from_tweet_id(tweet.id.as_u64()));
            let conversations = conversations_then.collect::<Vec<_>>().await;
            io::write_user_conversations_to_ron(&conversations, twitter_handle);
            conversations
        }
    }
}
//next up load conversations from data/conversations.ron
//if the last tweet in any conversation's id matched the input you can just  return that conversation

pub async fn load_conversation_from_tweet_id(tweet_id: u64) -> Vec<Tweet> {
    match fs::read_to_string("data/conversations.ron") {
        Ok(conversations_ron) => {
            println!("Loading conversation from \"data/conversations.ron\"");
            let conversations: Vec<Vec<Tweet>> = ron::from_str(&conversations_ron)
                .expect("Failed to parse conversations from \"data/conversations.ron\"");
            if conversations
                .iter()
                .any(|conversation| conversation[conversation.len() - 1].id.as_u64() == tweet_id)
            {
                conversations
                    .into_iter()
                    .filter(|conversation| {
                        conversation[conversation.len() - 1].id.as_u64() == tweet_id
                    })
                    .next()
                    .expect("Failed to get conversation")
            } else {
                println!("Loading conversation from API");
                let mut conversation =
                    api::get_twitter_conversation_from_tweet(load_tweet_from_id(tweet_id).await)
                        .await;
                io::write_conversation_to_ron(&mut conversation);
                conversation
            }
        }
        Err(_error) => {
            println!("Loading conversation from API");
            let mut conversation =
                api::get_twitter_conversation_from_tweet(load_tweet_from_id(tweet_id).await).await;
            io::write_conversation_to_ron(&mut conversation);
            conversation
        }
    }
}

pub async fn load_tweets_from_twitter_handle(twitter_handle: &str) -> Vec<Tweet> {
    match fs::read_to_string(format!("data/user-tweets_{twitter_handle}.ron")) {
        Ok(tweets) => ron::from_str(&tweets)
            .expect("Failed to load tweets from \"data/user-tweets_{twitter_handle}\""),
        Err(_error) => {
            let mut tweets =
                api::get_tweets_from_user(&load_user_from_twitter_handle(twitter_handle).await)
                    .await;
            io::write_user_tweets_to_ron(&tweets, twitter_handle);
            io::write_tweets_to_ron(&mut tweets);
            tweets
        }
    }
}
