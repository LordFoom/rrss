use std::collections::HashMap;

use log::info;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use tui_textarea::{CursorMove, TextArea};

#[derive(PartialEq, Eq, Default, Clone)]
pub enum AppState {
    #[default]
    Running,
    AddChannel,
    Stopped,
}

#[derive(PartialEq, Default, Clone)]
pub enum SelectedPane {
    #[default]
    Channels,
    Items,
}

#[derive(Default, Clone)]
pub struct App<'a> {
    pub channels: StatefulChannelList,
    pub current_items: StatefulItemList,
    pub state: AppState,
    pub selected_pane: SelectedPane,
    pub construct_items: bool,
    ///When this is set, display the text in an info popup until unset
    pub info_popup_text: Option<String>,
    pub error_popup_text: Option<String>,
    error_popup_thread_running: bool,
    pub add_channel_text_area: TextArea<'a>,
}

impl<'a> App<'a> {
    pub fn from(channels_vec: Vec<Channel>) -> Self {
        let channels = StatefulChannelList {
            state: ListState::default().with_offset(0),
            channels: channels_vec,
            last_selected: None,
        };
        Self {
            channels,
            current_items: StatefulItemList::default(),
            state: AppState::Running,
            selected_pane: SelectedPane::Channels,
            construct_items: true,
            info_popup_text: None,
            error_popup_text: None,
            error_popup_thread_running: false,
            add_channel_text_area: TextArea::default(),
        }
    }

    ///Maybe we fail to load some rss channels,
    ///so we need to display an error. To do that, we need error text.
    ///If we have errors, set the app's error text.
    pub fn set_loading_errors(&mut self, error_map: &HashMap<String, Option<String>>) {
        if !error_map.is_empty() {
            let error_txt = error_map
                .iter()
                .filter(|(_, v)| v.is_some())
                .map(|(_, v)| v.clone().get_or_insert("Unknown err".to_string()).clone())
                .fold(String::new(), |mut msg, curr_err_msg| {
                    msg.push_str(&curr_err_msg);
                    msg
                });

            self.error_popup_text = Some(error_txt);
        }
    }

    pub fn get_selected_channel(&self) -> Option<&Channel> {
        if let Some(idx) = self.channels.state.selected() {
            return self.channels.channels.get(idx);
        }
        None
    }

    pub fn update_selected_channel(&mut self, channel: &Channel) {
        self.channels.update_selected_channel(channel);
    }

    pub fn get_selected_item(&self) -> Option<&Item> {
        if let Some(idx) = self.current_items.state.selected() {
            return self.current_items.items.get(idx);
        }
        None
    }

    ///Graphically upwards from the current position
    ///If nothing selected, will select the last item
    pub fn select_up(&mut self) {
        match self.selected_pane {
            SelectedPane::Channels => self.select_up_channels(),
            SelectedPane::Items => self.select_up_items(),
        }
    }

    pub fn select_up_channels(&mut self) {
        let channels_len = self.num_channels();
        if channels_len == 0 {
            return;
        }
        let select_idx = if let Some(chnl_idx) = self.channels.state.selected() {
            if chnl_idx == 0 {
                //loop around
                channels_len - 1
            } else {
                chnl_idx - 1
            }
        } else {
            channels_len - 1
        };
        self.channels.state.select(Some(select_idx));
        //need to load items
        self.construct_items = true
    }

    pub fn select_up_items(&mut self) {
        let items_len = self.num_items();
        if items_len == 0 {
            return;
        }
        let select_idx = if let Some(item_idx) = self.current_items.state.selected() {
            if item_idx == 0 {
                //loop around
                items_len - 1
            } else {
                item_idx - 1
            }
        } else {
            items_len - 1
        };
        self.current_items.state.select(Some(select_idx));
    }

    ///Graphically upwards from the current position
    ///If nothing selected, will select the first item
    pub fn select_down(&mut self) {
        match self.selected_pane {
            SelectedPane::Channels => self.select_down_channels(),
            SelectedPane::Items => self.select_down_items(),
        }
    }

    pub fn select_down_channels(&mut self) {
        let channels_len = self.num_channels();
        if channels_len == 0 {
            return;
        }
        let select_idx = if let Some(chnl_idx) = self.channels.state.selected() {
            if chnl_idx == channels_len - 1 {
                //loop around
                0
            } else {
                chnl_idx + 1
            }
        } else {
            0
        };
        self.channels.state.select(Some(select_idx));
        //need to load items
        self.construct_items = true
    }

    pub fn select_down_items(&mut self) {
        let items_len = self.num_items();
        if items_len == 0 {
            return;
        }
        let select_idx = if let Some(item_idx) = self.current_items.state.selected() {
            if item_idx == items_len - 1 {
                //loop around
                0
            } else {
                item_idx + 1
            }
        } else {
            0
        };
        self.current_items.state.select(Some(select_idx));
    }

    pub fn change_selected_pane(&mut self) {
        match self.selected_pane {
            SelectedPane::Items => self.selected_pane = SelectedPane::Channels,
            SelectedPane::Channels => self.selected_pane = SelectedPane::Items,
        }
    }

    pub fn num_channels(&self) -> usize {
        self.channels.channels.len()
    }

    pub fn num_items(&self) -> usize {
        self.current_items.items.len()
    }

    pub fn content_pane_text(&self) -> String {
        if self.current_items.items.is_empty() {
            return "Nothing to display".to_string();
        }

        let idx = if let Some(idx) = self.current_items.state.selected() {
            idx
        } else {
            return "Nothing to display".to_string();
        };

        if let Some(item) = self.current_items.items.get(idx) {
            if let Some(description) = item.description.clone() {
                description
            } else {
                item.link
                    .clone()
                    .unwrap_or("Nothing to display".to_string())
                    .to_string()
            }
        } else {
            "Nothing to display".to_string()
        }
    }

    pub fn show_add_channel_dialog(&mut self) {
        self.state = AppState::AddChannel
    }

    pub fn unshow_add_channel_dialog(&mut self) {
        self.clear_add_channel_text_area();
        self.state = AppState::Running
    }

    ///Add the text in dialog as a channel
    pub fn add_channel(&mut self) -> Option<String> {
        //expect a single line
        let hopefully_a_channel = self.add_channel_text_area.lines()[0].clone();
        if hopefully_a_channel.is_empty() {
            return None;
        }
        let channel_link = ChannelLink {
            href: Some(hopefully_a_channel.clone()),
            value: None,
        };
        let link = vec![channel_link.clone()];
        let channel_to_add = Channel {
            title: hopefully_a_channel,
            link,
            ..Default::default()
        };
        self.channels.channels.push(channel_to_add);
        let num_channels = self.num_channels();
        self.clear_add_channel_text_area();
        //select the just added channel
        self.channels.state.select(Some(num_channels - 1));
        //we no longer wish to display the textarea
        self.state = AppState::Running;
        channel_link.href
    }

    pub fn set_add_channel_contents(&mut self, contents: &str) {
        self.add_channel_text_area.insert_str(contents);
    }

    pub fn clear_add_channel_text_area(&mut self) {
        self.add_channel_text_area.move_cursor(CursorMove::Head);
        while self.add_channel_text_area.delete_line_by_end() {
            self.add_channel_text_area
                .move_cursor(tui_textarea::CursorMove::Head);
        }
        self.add_channel_text_area.move_cursor(CursorMove::End);

        while self.add_channel_text_area.delete_line_by_head() {
            self.add_channel_text_area
                .move_cursor(tui_textarea::CursorMove::End);
        }
    }

    pub fn is_showing_untimed_error(&self) -> bool {
        self.error_popup_text.is_some() && !self.error_popup_thread_running
    }

    ///Clear out info and error popups which may be being displayed
    pub fn reset_all_popups(&mut self) {
        self.info_popup_text = None;
        self.error_popup_text = None;
        self.error_popup_thread_running = false;
        info!("Cleared out popup text!")
    }
}

#[derive(Default, Clone)]
pub struct StatefulChannelList {
    pub state: ListState,
    pub channels: Vec<Channel>,
    pub last_selected: Option<usize>,
}

impl StatefulChannelList {
    pub fn update_selected_channel(&mut self, channel: &Channel) {
        if let Some(idx) = self.state.selected() {
            self.channels[idx] = channel.clone();
        } else {
            self.channels.push(channel.clone());
        }
    }
}

///Intended to display a channels items in a pane
#[derive(Default, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Channel {
    pub title: String,
    pub link: Vec<ChannelLink>,
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    #[serde(rename = "item")]
    pub items: Vec<Item>,
    pub image: Option<Vec<Image>>,
}

///Structure to hold the various link elements  in the returned xml
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChannelLink {
    pub href: Option<String>,
    ///Actual text of the link
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

impl Channel {
    ///Get the rss url for the channel
    pub fn get_link(&self) -> String {
        info!("Getting the link...");
        if self.link.is_empty() {
            return "UNKNOWN".to_string();
        }
        let mut return_link = String::new();
        for link in self.link.clone() {
            if let Some(href) = link.href {
                //if href.ends_with("xml") || href.ends_with("rss") {
                return_link = href
                //}
            }
        }
        info!("We are returning {return_link}");
        return_link
    }

    ///Set the url of the channel
    pub fn set_link(&mut self, txt: &str) {
        //clear out old items
        if !self.link.is_empty() {
            self.link = Vec::new();
        }
        let channel_link = ChannelLink {
            href: Some(txt.to_string()),
            value: None,
        };

        self.link.push(channel_link);
    }
}

///News items in the channel
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    // pub title: String,
    // pub link: String,
    pub title: Option<Vec<String>>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub enclosure: Option<Enclosure>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
}

impl Item {
    pub fn get_title(&self) -> String {
        if let Some(titles) = &self.title {
            if let Some(title) = titles.first() {
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

mod test {
    /* reference xml for our test
    <atom:link href="https://feeds.buzzsprout.com/2042709.rss" rel="self" type="application/rss+xml" />
    <atom:link href="https://pubsubhubbub.appspot.com/" rel="hub" xmlns="http://www.w3.org/2005/Atom" />
    <title>Between Two Cairns</title>
    <lastBuildDate>Thu, 30 May 2024 14:30:13 -0400</lastBuildDate>
    <link>https://www.buzzsprout.com/2042709</link>
    <language>en-us</language>
    */

    #[test]
    pub fn test_get_link() {
        // let mut links = ""
    }
}
