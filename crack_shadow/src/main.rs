use anyhow::Result;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser)]
    shadow: Option<String>,

    #[clap(short, long, value_parser)]
    wordlist: Option<String>,
}

const WORDLIST: &str = "wordlist";

fn main() -> Result<()> {
    let args = Args::parse();

    let shadow = if let Some(shadow) = args.shadow {
        shadow
    } else {
        "shadow".to_string()
    };

    let wordlist = if let Some(wordlist) = args.wordlist {
        wordlist
    } else {
        WORDLIST.to_string()
    };

    let list = lib::crack_shadow(&shadow, &wordlist)?;

    for (user, pwrd) in list {
        println!("Found {} :: {}", user, pwrd);
    }

    Ok(())
}
