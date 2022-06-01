use std::fs::{self};

pub fn string_from_ron(file_path: &str) -> Result<String, std::io::Error> {
    println!("Reading file: \"{file_path}\"");
    fs::read_to_string(file_path)
}

pub fn tweets_string_from_ron() -> Result<String, std::io::Error> {
    string_from_ron("data/tweets.ron")
}

pub fn conversations_string_from_ron() -> Result<String, std::io::Error> {
    string_from_ron("data/conversations.ron")
}

pub fn user_info_string_from_ron(twitter_handle: &str) -> Result<String, std::io::Error> {
    let file_path: &str = &format!("data/user-info_{twitter_handle}.ron");
    string_from_ron(file_path)
}

pub fn user_tweets_string_from_ron(twitter_handle: &str) -> Result<String, std::io::Error> {
    let file_path: &str = &format!("data/user-tweets_{twitter_handle}.ron");
    string_from_ron(file_path)
}

pub fn user_conversations_string_from_ron(twitter_handle: &str) -> Result<String, std::io::Error> {
    let file_path: &str = &format!("data/user-conversations_{twitter_handle}.ron");
    string_from_ron(file_path)
}

pub fn users_string_from_ron() -> Result<String, std::io::Error> {
    let file_path: &str = &format!("data/users.ron");
    string_from_ron(file_path)
}
