use anyhow::Result;
use clap::Parser;
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

    let rss: Rss = from_str(&txt).unwrap();
    println!("Rss: {:?}", rss);
    Ok(())
}
