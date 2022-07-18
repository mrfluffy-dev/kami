mod anime;
mod helpers;
mod ln;

use anime::anime::anime_stream;
use colored::Colorize;
use ln::{scraper::get_ln_next_page, ln::ln_read};
use ln::search::search_ln;

use crate::anime::{
    player::open_video,
    scraper::{anime_ep_range, anime_link, anime_names},
};
use crate::helpers::take_input::{int_input, string_input};
use crate::ln::{menu::chapter_selector, open_text::open_bat, scraper::get_full_text};

fn main() {
    let mut _arg = String::new();
    if std::env::args().len() > 1 {
        _arg = std::env::args().nth(1).unwrap();
    } else {
        println!("anime:\t\t{}", format_args!("{}", "a".red()));
        println!("light novel:\t{}", format_args!("{}", "l".red()));
        println!(
            "you can add the name of the anime you want to watch after the {} argument",
            format_args!("{}", "a".red())
        );
        //kill the program
        std::process::exit(0);
    }
    if _arg == "l" {
        ln_read();
    } else if _arg == "a" {
        anime_stream(true)
    } else {
        println!("Invalid argument");
    }
}

fn page_selector(ln_id: &str, selected_page: u32) -> String {
    get_ln_next_page(ln_id, &selected_page.to_string())
}
