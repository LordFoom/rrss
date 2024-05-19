use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq)]
pub enum AppState {
    Running,
    Stopped,
}

pub struct App {
    pub channels: StatefulChannelList,
    pub current_items: Option<StatefulChannelList>,
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
            current_items: None,
            state: AppState::Running,
        }
    }

    pub fn get_selected_channel(&self) -> Option<&Channel> {
        if let Some(idx) = self.channels.state.selected() {
            return self.channels.channels.get(idx);
        }
        None
    }

    ///Graphically upwards from the current position
    ///If nothing selected, will select the last item
    pub fn channel_select_up(&mut self) {
        let channel_len = self.num_channels();
        let select_idx = if let Some(chnl_idx) = self.channels.state.selected() {
            if chnl_idx == 0 {
                //loop around
                channel_len - 1
            } else {
                chnl_idx - 1
            }
        } else {
            channel_len - 1
        };
        let _ = self.channels.state.select(Some(select_idx));
    }

    ///Graphically upwards from the current position
    ///If nothing selected, will select the first item
    pub fn channel_select_down(&mut self) {
        let channel_len = self.num_channels();
        let select_idx = if let Some(chnl_idx) = self.channels.state.selected() {
            if chnl_idx == channel_len - 1 {
                //loop around
                0
            } else {
                chnl_idx + 1
            }
        } else {
            0
        };
        let _ = self.channels.state.select(Some(select_idx));
    }

    pub fn num_channels(&self) -> usize {
        self.channels.channels.len()
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
    pub fn from(channel: &Channel) -> Self {
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

impl Item {
    pub fn get_title(&self) -> String {
        if let Some(titles) = &self.title {
            if let Some(title) = titles.get(0) {
                return title.clone();
            }
        }
        String::from("None")
    }
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
