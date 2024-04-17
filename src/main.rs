use clap::Parser;

mod model;

#[derive(Parser, Debug)]
struct Args {
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = args.url;
    let result = reqwest::get(url).await;
    println!("{:?}", result);
}
