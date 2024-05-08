use anyhow::{Context, Result};
use api::fetch_rss_feed;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use display::display_channel;
use log::{debug, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use model::App;
use ratatui::{backend::Backend, Terminal};
use tui::{restore_terminal, setup_terminal, ui};

mod api;
mod display;
mod model;
mod tui;

#[derive(Parser, Debug)]
struct Args {
    urls: Vec<String>,
    #[arg(short, long)]
    verbose: bool,
}

///Set up logging to the file, if enabled
pub fn init_logging(verbose: bool) -> Result<()> {
    // let stdout = ConsoleAppender::builder().build();

    let lvl_filter = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    //TODO make this configurable
    let file_path = "./rrss.log";

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d_%H:%M:%S)(utc)}-{h({l}:{f}>{L}    {m})}\n",
        )))
        .build(file_path)?;

    let config = Config::builder()
        // .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        // .build(Root::builder().appender("stdout").build(lvl_filter))?;
        .build(Root::builder().appender("log_file").build(lvl_filter))?;

    log4rs::init_config(config)?;
    Ok(())
}

//TODO Do multiple file urls from cli
//TODO Allow url in config file
//TODO make a tui
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_logging(args.verbose)?;

    let mut term = setup_terminal().context("Failed to setup terminal")?;

    let mut channels = Vec::new();
    for url in args.urls {
        if let Some(channel) = fetch_rss_feed(&url).await? {
            channels.push(channel);
            // display_channel(&channel);
        } else {
            debug!("No rss channel found...");
        }
    }

    let mut app = App::from(channels);
    run_app(&mut term, &mut app)?;
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
    restore_terminal(&mut term).context("Failed to restore terminal")?;
    Ok(())
}

///Run run run the app merrily down the bitstream
fn run_app<B: Backend>(term: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        term.draw(|f| ui(f, app).expect("Could not draw the ui"))?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
