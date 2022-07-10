use crate::anime::anime::anime_stream;
use crate::anime::scraper::{anime_ep_range, anime_link, anime_names};
use crate::ln::ln::ln;
use crate::ln::menu::chapter_selector;
use crate::ln::open_text::open_bat;
use crate::ln::scraper::get_full_text;
use crate::{
    anime::player::open_video,
    helpers::take_input::{int_input, string_input, u16_input},
    ln::search::search_ln,
};
use colored::Colorize;
use ln::scraper::get_ln_next_page;
mod anime;
mod helpers;
mod ln;

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
        ln();
    } else if _arg == "a" {
        anime_stream()
    } else {
        println!("Invalid argument");
    }
}

fn page_selector(ln_id: &str, selected_page: u32) -> String {
    get_ln_next_page(ln_id, &selected_page.to_string())
}
