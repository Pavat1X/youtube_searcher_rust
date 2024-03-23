use clap::Parser;
#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() {
    let args: Cli = Cli::parse();
}
