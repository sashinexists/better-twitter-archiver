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

#[get("/user/<handle>")]

#[get("/user/<handle>/info")]

#[get("/user/<handle>/tweets")]

#[get("/user/<handle>/conversations")]

#[get("/search?<query>")]

"#
}

#[get("/tweet/<id>")]
async fn tweet_by_id(id: u64) -> String {
    ron::to_string(&app::get_tweet_by_id(id).await).expect("Failed to serve tweet from id")
}

//for your purposes a conversation id might be the *last* tweet id in the conversation
#[get("/conversation/<id>")]
async fn conversation_by_id(id: u64) -> String {
    ron::to_string(&app::get_twitter_conversation_from_tweet(app::get_tweet_by_id(id).await).await)
        .expect("Failed to serve twitter conversation")
}

//maybe this could be a tuple for a tweed id and a conversation
#[get("/conversation/<id>/<tweet_id>")]
async fn tweet_in_conversation_by_id(id: u64, tweet_id: u64) -> String {
    ron::to_string(&(
        tweet_id,
        &app::get_twitter_conversation_from_tweet(app::get_tweet_by_id(id).await).await,
    ))
    .expect("Failed to serve twitter conversation")
}
// will just get info on a user
#[get("/user/<handle>")]
async fn user_by_handle(handle: &str) -> String {
    ron::to_string(&app::get_user_by_twitter_handle(handle).await)
        .expect("Failed to serve user from id")
}
//exact same as get user_by_handle
#[get("/user/<handle>/info")]
async fn user_info_by_handle(handle: &str) -> String {
    ron::to_string(&app::get_user_by_twitter_handle(handle).await)
        .expect("Failed to serve user from id")
}

//will bet a user's tweets, for now the recent ten
#[get("/user/<handle>/tweets")]
async fn tweets_by_user(handle: &str) -> String {
    ron::to_string(&app::get_tweets_from_user(&app::get_user_by_twitter_handle(handle).await).await)
        .expect("Failed to serve user's tweets from this twitter handle")
}

//will get a user's conversations
#[get("/user/<handle>/conversations")]
async fn conversations_by_user(handle: &str) -> String {
    ron::to_string(
        &app::load_conversations(
            app::get_tweets_from_user(&app::get_user_by_twitter_handle(&handle).await).await,
        )
        .await,
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
        .mount("/", routes![conversations_by_user])
        .mount("/", routes![tweets_by_user])
        .mount("/", routes![user_info_by_handle])
        .mount("/", routes![user_by_handle])
        .mount("/", routes![tweet_in_conversation_by_id])
        .mount("/", routes![conversation_by_id])
        .mount("/", routes![tweet_by_id])
        .mount("/", routes![index])
}
