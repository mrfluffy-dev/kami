mod anime;
mod helpers;
mod ln;
mod ui;

use anime::anime::anime_ui;
use ln::ln::ln_ui;

use crate::anime::trackers::*;
use crate::get_token;
use crate::helpers::take_input::{int_input, string_input};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use anime mode
    #[arg(short, long)]
    anime: bool,
    /// Use light novel mode
    #[arg(short, long)]
    ln: bool,

    #[arg(short, long, default_value_t = 0)]
    chapter: u32,

    #[arg(short = 'C', long, default_value = "0")]
    cast: String,

    /// Provider for anime or light novel
    #[arg(short = 'r', long, default_value = "gogo")]
    provider: String,

    /// Text renderer for light novel
    #[arg(short = 'R', long, default_value = "bat")]
    reader: String,
}

fn main() {
    let mut args = Args::parse();

    if args.anime == false && args.ln == false {
        println!("1:    Anime");
        println!("2:    Light Novel");

        let a = int_input("pick your poison: ");
        match a {
            1 => args.anime = true,
            2 => args.ln = true,
            _ => println!("invalid option. "),
        };
    }
    if args.anime == true && args.ln == true {
        println!("you can only use one of the arguments at a time");
        std::process::exit(0);
    }

    if args.ln == true {
        //ln_read(&search, chapter);
        _ = ln_ui(args.chapter, args.reader);
    } else if args.anime == true {
        //anime_stream(search, episode, resume);

        let token = get_token();
        _ = anime_ui(token, args.provider, (args.cast == "0", args.cast));
    } else {
        println!("Invalid argument");
    }
}
