use anyhow::{anyhow, Result};
use log::{debug, info, warn};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Client, StatusCode,
};

use crate::model::{Channel, Rss};
use serde::Deserialize;
///Get the contents of an rss feed
pub async fn fetch_rss_feed(url: &str) -> Result<Option<Channel>> {
    let client = Client::new();
    let mut header_map = HeaderMap::new();
    header_map.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8"));
    let result = client.get(url).headers(header_map).send().await?;

    //let result = reqwest::get(url).await?;
    let response_status = result.status();
    if response_status != StatusCode::OK {
        return Err(anyhow!(response_status.to_string()));
    }
    info!("We got back this status: {}", response_status.to_string());
    let txt = result.text().await?;
    info!("Text returned from the url: {}", txt);
    let mut de = serde_xml_rs::Deserializer::new_from_reader(txt.as_bytes())
        .non_contiguous_seq_elements(true);
    match Rss::deserialize(&mut de) {
        Ok(rss) => Ok(Some(rss.channel)),
        Err(e) => panic!("Failed to deserialize! {}", e),
    }
}
