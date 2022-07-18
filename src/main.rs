use crate::anime::anime::anime_stream;
use crate::anime::scraper::{anime_ep_range, anime_link, anime_names};
use crate::ln::ln::ln_read;
use crate::ln::menu::chapter_selector;
use crate::ln::open_text::open_bat;
use crate::ln::scraper::get_full_text;
use crate::{
    anime::player::open_video,
    helpers::take_input::{int_input, string_input},
    ln::search::search_ln,
};
use colored::Colorize;
use ln::scraper::get_ln_next_page;
mod anime;
mod helpers;
mod ln;

fn main() {
    let mut help = false;
    let mut anime = false;
    let mut ln = false;
    for arg in std::env::args() {
        if arg == "--help" || arg == "-h" {
            help = true;
        }
        if arg == "--anime" || arg == "-a" {
            anime = true;
        }
        if arg == "--ln" || arg == "-l" {
            ln = true;
        }
    }
    if help == true{
        print_help();
    }
    if anime == false && ln == false {
        print_help();
    }
    if anime == true && ln == true {
        println!("you can only use one of the arguments at a time");
        std::process::exit(0);
    }
    if ln == true {
        ln_read();
    } else if anime == true {
        anime_stream(true)
    } else {
        println!("Invalid argument");
    }
}

fn page_selector(ln_id: &str, selected_page: u32) -> String {
    get_ln_next_page(ln_id, &selected_page.to_string())
}

fn print_help(){
    println!("anime:\t\t{}", format_args!("{}", "-a --anime".red()));
    println!("light novel:\t{}", format_args!("{}", "-l --ln".red()));
    println!(
        "you can add the name of the anime you want to watch after the {} argument",
        format_args!("{}", "-a --anime".red())
    );
    println!("help:\t\t{}", format_args!("{}", "-h --help".red()));
    //kill the program
    std::process::exit(0);
}
