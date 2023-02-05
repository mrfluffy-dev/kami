mod anime;
mod helpers;
mod ln;
mod ui;

#[macro_use]
extern crate lazy_static;

use anime::anime::anime_ui;
use ln::ln::ln_ui;

use crate::anime::trackers::*;
use crate::get_token;
use crate::helpers::take_input::{int_input, string_input};

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum KamiMode {
    #[command(name = "ln", about = "Search Light Novel to read.")]
    LightNovel,
    #[command(about = "Search Anime to read.")]
    Anime,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "A scraper to read light novels and watch anime in your terminal.", long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Option<KamiMode>,

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
    let args = Args::parse();

    let mode = match &args.mode {
        None => {
            println!("1:    Anime");
            println!("2:    Light Novel");

            let opt = int_input("pick your poison: ");

            match opt {
                1 => &KamiMode::Anime,
                2 => &KamiMode::LightNovel,
                _ => {
                    println!("invalid option.");
                    std::process::exit(0);
                }
            }
        }
        Some(m) => m,
    };

    let _ = match mode {
        &KamiMode::LightNovel => ln_ui(args.chapter, args.reader),
        &KamiMode::Anime => {
            let token = get_token();
            anime_ui(token, args.provider, (args.cast == "0", args.cast))
        }
    };
}
