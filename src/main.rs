use clap::Parser;

mod model;

#[derive(Parser, Debug)]
struct Args {
    url: String,
}

fn main() {
    let args = Args::parse();
}
