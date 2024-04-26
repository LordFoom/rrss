use anyhow::Result;
use api::fetch_rss_feed;
use clap::Parser;
use display::display_channel;
use log::Level;
use log4rs::append::console::ConsoleAppender;

mod api;
mod display;
mod model;

#[derive(Parser, Debug)]
struct Args {
    urls: Vec<String>,
    #[arg(short, long)]
    verbose: Option<bool>,
}

//TODO Do multiple file urls from cli
//TODO Allow url in config file
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    for url in args.urls {
        if let Some(channel) = fetch_rss_feed(&url).await? {
            display_channel(&channel);
        } else {
            println!("No rss channel found...");
        }
    }
    // let result = reqwest::get(url).await?;
    // println!("{:?}", result);
    // let txt = result.text().await?;
    //
    // println!("Text: {}", txt);
    //
    // let mut de = serde_xml_rs::Deserializer::new_from_reader(txt.as_bytes())
    //     .non_contiguous_seq_elements(true);
    // let rss = Rss::deserialize(&mut de).unwrap();
    // println!("Rss: {:?}", rss);
    Ok(())
}
