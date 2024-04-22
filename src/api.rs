use anyhow::Result;

use crate::model::{Channel, Rss};
use serde::Deserialize;
///Get the contents of an rss feed
pub async fn fetch_rss_feed(url: &str) -> Result<Option<Channel>> {
    let result = reqwest::get(url).await?;
    let txt = result.text().await?;
    println!("Text returned from the url: {}", txt);
    let mut de = serde_xml_rs::Deserializer::new_from_reader(txt.as_bytes())
        .non_contiguous_seq_elements(true);
    match Rss::deserialize(&mut de) {
        Ok(rss) => return Ok(Some(rss.channel)),
        Err(e) => panic!("Failed to deserialize! {}", e),
    }
}
