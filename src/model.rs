use serde::{Deserialize, Serialize};

///Big rss wrapping tag
#[derive(Serialize, Deserialize)]
struct Rss {
    version: String,
}

///Rss Channel
#[derive(Serialize, Deserialize)]
struct Channel {
    title: String,
    link: String,
    description: String,
}

///News items in the channel
#[derive(Serialize, Deserialize)]
struct Item {
    title: String,
    link: String,
    description: String,
}
