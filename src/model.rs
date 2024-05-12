use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq)]
pub enum AppState {
    RUNNING,
    STOPPED,
}

pub struct App {
    pub channels: StatefulChannelList,
    pub state: AppState,
}

impl App {
    pub fn from(channels_vec: Vec<Channel>) -> Self {
        let channels = StatefulChannelList {
            state: ListState::default().with_offset(0),
            channels: channels_vec,
            last_selected: None,
        };
        Self {
            channels,
            state: AppState::RUNNING,
        }
    }
}

pub struct StatefulChannelList {
    pub state: ListState,
    pub channels: Vec<Channel>,
    pub last_selected: Option<usize>,
}

///Intended to display a channels items in a pane
pub struct StatefulItemList {
    pub state: ListState,
    pub items: Vec<Item>,
    pub last_selected: Option<usize>,
}

impl StatefulItemList {
    pub fn from_channel(channel: &Channel) -> Self {
        Self {
            state: ListState::default().with_offset(0),
            items: channel.items.clone(),
            last_selected: None,
        }
    }
}
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
    pub image: Option<Vec<Image>>,
}

///News items in the channel
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Image {
    pub url: Option<String>,
    pub href: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
}
