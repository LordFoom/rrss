use serde::{Deserialize, Serialize};

///Big rss wrapping tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Rss {
    pub version: String,
    pub channel: Channel,
}

///Rss Channel
#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub title: String,
    pub link: Vec<String>,
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

///News items in the channel
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    // pub title: String,
    // pub link: String,
    pub title: Option<Vec<String>>,
    pub link: Option<String>,
    pub description: Option<String>,
    //TODO this needs to be its own object
    pub enclosure: Option<Enclosure>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Enclosure {
    pub url: String,
    pub length: String,
    #[serde(rename = "type")]
    pub enclosure_type: String,
}
