use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde_xml_rs::from_str;

mod model;
use model::Rss;

#[derive(Parser, Debug)]
struct Args {
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let url = args.url;
    let result = reqwest::get(url).await?;
    println!("{:?}", result);
    let txt = result.text().await?;

    println!("Text: {}", txt);

    let mut de = serde_xml_rs::Deserializer::new_from_reader(txt.as_bytes())
        .non_contiguous_seq_elements(true);
    let rss = Rss::deserialize(&mut de).unwrap();
    println!("Rss: {:?}", rss);
    println!("Channel: {:?}", rss.channel);
    Ok(())
}
