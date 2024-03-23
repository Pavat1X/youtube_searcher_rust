use clap::Parser;
use open;
#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main() {
    let url = "https://www.youtube.com/results?search_query=";
    let args: Cli = Cli::parse();
    let search = url.to_owned() + &args.pattern;
    match open::that(search) {
        Ok(()) => println!("success!"),
        Err(err) => eprintln!("error: {}", err),
    }
}
