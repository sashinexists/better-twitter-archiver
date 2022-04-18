#[macro_use]
extern crate rocket;
use dotenvy::dotenv;

pub mod app;

#[get("/")]
fn index() -> &'static str {
    r#"Welcome to a better twitter archiver! 

Available routes:

#[get("/tweet/<id>")]

#[get("/conversation/<id>")]

#[get("/conversation/<id>/<tweet_id>")]

#[get("/user/<twitter_handle>")]

#[get("/user/<twitter_handle>/info")]

#[get("/user/<twitter_handle>/tweets")]

#[get("/user/<twitter_handle>/conversations")]

#[get("/search?<query>")]

"#
}

#[get("/tweet/<id>")]
async fn tweet_by_id(id: u64) -> String {
    ron::ser::to_string_pretty(
        &app::load_tweet_from_id(id).await,
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve tweet from id")
}

//for your purposes a conversation id might be the *last* tweet id in the conversation
#[get("/conversation/<id>")]
async fn conversation_by_id(id: u64) -> String {
    ron::ser::to_string_pretty(
        &app::get_twitter_conversation_from_tweet(app::load_tweet_from_id(id).await).await,
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve twitter conversation")
}

//maybe this could be a tuple for a tweed id and a conversation
#[get("/conversation/<id>/<tweet_id>")]
async fn tweet_in_conversation_by_id(id: u64, tweet_id: u64) -> String {
    ron::ser::to_string_pretty(
        &(
            tweet_id,
            &app::get_twitter_conversation_from_tweet(app::load_tweet_from_id(id).await).await,
        ),
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve twitter conversation")
}
// will just get info on a user
#[get("/user/<twitter_handle>")]
async fn user_by_twitter_handle(twitter_handle: &str) -> String {
    ron::ser::to_string_pretty(
        &app::load_user_from_twitter_handle(twitter_handle).await,
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve user from id")
}
//exact same as get user_by_twitter_handle
#[get("/user/<twitter_handle>/info")]
async fn user_info_by_twitter_handle(twitter_handle: &str) -> String {
    ron::ser::to_string_pretty(
        &app::load_user_from_twitter_handle(twitter_handle).await,
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve user from id")
}

//will bet a user's tweets, for now the recent ten
#[get("/user/<twitter_handle>/tweets")]
async fn tweets_by_user(twitter_handle: &str) -> String {
    ron::ser::to_string_pretty(
        &app::load_tweets_from_twitter_handle(twitter_handle).await,
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve user's tweets from this twitter handle")
}

//will get a user's conversations
#[get("/user/<twitter_handle>/conversations")]
async fn conversations_by_twitter_handle(twitter_handle: &str) -> String {
    ron::ser::to_string_pretty(
        &app::load_conversations_from_twitter_handle(twitter_handle).await,
        ron::ser::PrettyConfig::new(),
    )
    .expect("Failed to serve user's tweets from this twitter handle")
}
//in the url the query will look like "/search?query=whatever"
#[get("/search?<query>")]
fn search(query: &str) -> String {
    format!(
        "This route will run a search of \"{}\" through all the tweets locally stored",
        query
    )
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![search])
        .mount("/", routes![conversations_by_twitter_handle])
        .mount("/", routes![tweets_by_user])
        .mount("/", routes![user_info_by_twitter_handle])
        .mount("/", routes![user_by_twitter_handle])
        .mount("/", routes![tweet_in_conversation_by_id])
        .mount("/", routes![conversation_by_id])
        .mount("/", routes![tweet_by_id])
        .mount("/", routes![index])
}
