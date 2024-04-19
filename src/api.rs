use anyhow::Result;

use crate::model::{Channel, Rss};
use serde::{Deserialize, Serialize};
///Get the contents of an rss feed
pub async fn fetch_rss_feed(url: &str) -> Result<Option<Channel>> {
    let result = reqwest::get(url).await?;
    let txt = result.text().await?;
    let mut de = serde_xml_rs::Deserializer::new_from_reader(txt.as_bytes())
        .non_contiguous_seq_elements(true);
    if let Ok(rss) = Rss::deserialize(&mut de) {
        return Ok(Some(rss.channel));
    } else {
        return Ok(None);
    }
}
