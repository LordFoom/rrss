use anyhow::{Context, Result};
use api::fetch_rss_feed;
use clap::{ArgGroup, Parser};
use color_eyre::config::HookBuilder;
use config::load_config;
use log::{debug, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use model::App;
use tui::{restore_terminal, run_app, setup_terminal};

mod api;
mod config;
mod display;
mod model;
mod tui;

#[derive(Parser, Debug)]
#[command(group = ArgGroup::new("exclusive").args(&["urls", "file"]))]
struct Args {
    ///Optional list of urls to load up for the reader
    urls: Vec<String>,
    #[arg(short, long)]
    verbose: bool,
    ///Optional file with toml of channels to use
    #[arg(short, long)]
    file: Option<String>,
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

fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        if let Err(err) = restore_terminal() {
            eprintln!("Unable to restore terminal ${err}");
        }
        error(e)
    }))?;

    std::panic::set_hook(Box::new(move |info| {
        if let Err(err) = restore_terminal() {
            eprintln!("Unable to restore terminal ${err}");
        }
        panic(info)
    }));

    Ok(())
}

//TODO Allow url in config file
//TODO populate items from channel selection
//TODO only fetch items for channel when triggered
//TODO display item
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_error_hooks()?;

    init_logging(args.verbose)?;

    let mut term = setup_terminal().context("Failed to setup terminal")?;

    let mut channels = Vec::new();
    for url in args.urls.clone() {
        //this we should make async, so we can start up and it does it in the background...?
        if let Some(channel) = fetch_rss_feed(&url).await? {
            channels.push(channel);
            // display_channel(&channel);
        } else {
            debug!("No rss channel found...");
        }
    }

    //if no urls are passed in, we look at the config
    if args.urls.clone().is_empty() {
        //see if there is config to load
        let maybe_config = load_config(args.file)?;
        if let Some(cfg) = maybe_config {}
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
    // restore_terminal(&mut term).context("Failed to restore terminal")?;
    restore_terminal().context("Failed to restore terminal")?;
    Ok(())
}
