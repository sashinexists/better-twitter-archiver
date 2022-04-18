#[macro_use]
extern crate rocket;
use dotenvy::dotenv;

pub mod app;

#[get("/")]
fn index() -> &'static str {
    "Welcome to a better twitter archiver"
}

#[get("/tweet/<id>")]
async fn tweet_by_id(id: u64) -> String {
    ron::to_string(&app::get_tweet_by_id(id).await).expect("Failed to serve tweet from id")
}

#[get("/conversation/<id>")]
fn conversation_by_id(id: u64) -> String {
    format!("This routes will get a conversation of id  {}!", id)
}

#[get("/conversation/<id>/<tweet_id>")]
fn tweet_in_conversation_by_id(id: u64, tweet_id: u64) -> String {
    format!(
        "This routes will get a conversation of id  {id} with the tweet of id {tweet_id} highlighted"
    )
}

#[get("/user/<handle>")]
fn user_by_handle(handle: &str) -> String {
    format!(
        "This route will get details on the user with the handle {}",
        handle
    )
}

#[get("/user/<handle>/info")]
fn user_info_by_handle(handle: &str) -> String {
    format!(
        "This route will get info on the user with the handle {}",
        handle
    )
}

#[get("/user/<handle>/tweets")]
fn tweets_by_user(handle: &str) -> String {
    format!(
        "This route will get the tweets on the user with the handle {}",
        handle
    )
}

#[get("/user/<handle>/conversations")]
fn conversations_by_user(handle: &str) -> String {
    format!(
        "This route will get the conversations on the user with the handle {}",
        handle
    )
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
