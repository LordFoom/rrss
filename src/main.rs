use anyhow::Result;
use api::fetch_rss_feed;
use clap::Parser;
use display::display_channel;
use log::{debug, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config,
};

mod api;
mod display;
mod model;

#[derive(Parser, Debug)]
struct Args {
    urls: Vec<String>,
    #[arg(short, long)]
    verbose: bool,
}

pub fn init_logging(verbose: bool) -> Result<()> {
    let stdout = ConsoleAppender::builder().build();

    let lvl_filter = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(lvl_filter))?;

    log4rs::init_config(config)?;
    Ok(())
}

//TODO Do multiple file urls from cli
//TODO Allow url in config file
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_logging(args.verbose)?;

    for url in args.urls {
        if let Some(channel) = fetch_rss_feed(&url).await? {
            display_channel(&channel);
        } else {
            debug!("No rss channel found...");
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
