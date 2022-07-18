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
    let mut help = false;
    let mut anime = false;
    let mut ln = false;
    let mut chapter: u32 = 0;
    let mut episode: u32 = 0;
    //let search = option string
    let mut search = String::new();
    let mut count = 0;
    for arg in std::env::args() {
        if arg == "--help" || arg == "-h" {
            help = true;
        }
        if arg == "--anime" || arg == "-a" {
            anime = true;
            //look at the next argument and see if it is a search term
            if let Some(arg) = std::env::args().nth(count + 1) {
                search = arg;
            }
        }
        if arg == "--ln" || arg == "-l" {
            ln = true;
            if let Some(arg) = std::env::args().nth(count + 1) {
                search = arg;
            }
        }
        if arg == "--chapter" || arg == "-c" {
            if let Some(arg) = std::env::args().nth(count + 1) {
                chapter = arg.parse::<u32>().unwrap();
            }else{
                chapter = 0;
            }
        }
        if arg == "--episode" || arg == "-e" {
            if let Some(arg) = std::env::args().nth(count + 1) {
                episode = arg.parse::<u32>().unwrap();
            }else{
                episode = 0;
            }
        }
        count += 1;
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
        ln_read(&search, chapter);
    } else if anime == true {
        anime_stream(search, episode);
    } else {
        println!("Invalid argument");
    }
}

fn page_selector(ln_id: &str, selected_page: u32) -> String {
    get_ln_next_page(ln_id, &selected_page.to_string())
}

fn print_help(){
    println!("anime:\t\t{}", format_args!("{}", "-a --anime".red()));
    println!("{}", "after this^^^ argument you can either enter a search term".green());
    println!("{}", "for exaple kami -a \"one piece\"");
    //print blank line
    println!("");
    println!("episode:\t{}", format_args!("{}", "-e --episode".red()));
    println!("{}", "after this^^^ argument you can either enter a chapter number".green());
    println!("{}", "for exaple kami -c 200");
    //print blank line
    println!("");
    println!("light novel:\t{}", format_args!("{}", "-l --ln".red()));
    println!("{}", "after this^^^ argument you can either enter a search term".green());
    println!("{}", "for exaple kami -l \"one piece\"");
    //print blank line
    println!("");
    println!("chapter:\t{}", format_args!("{}", "-c --chapter".red()));
    println!("{}", "after this^^^ argument you can either enter a chapter number".green());
    println!("{}", "for exaple kami -c 200");
    //print blank line
    println!("");
    println!("help:\t\t{}", format_args!("{}", "-h --help".red()));
    //kill the program
    std::process::exit(0);
}
