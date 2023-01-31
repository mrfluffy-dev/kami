mod anime;
mod helpers;
mod ln;

use anime::anime::anime_ui;
use colored::Colorize;
//use ln::ui::ln_ui;
use ln::ln::ln_ui;

use crate::anime::{player::*, scraper::*, trackers::*};
use crate::get_token;
use crate::helpers::take_input::{int_input, string_input};
fn main() {
    let mut help = false;
    let mut anime = false;
    let mut ln = false;
    let mut chapter: u32 = 0;
    //let search = option string
    let mut count = 0;
    let mut provider: String = "gogo".to_string();
    let mut cast = (false, "0".to_string());
    for arg in std::env::args() {
        if arg == "--help" || arg == "-h" {
            help = true;
        }
        if arg == "--anime" || arg == "-a" {
            anime = true;
        }
        if arg == "--provider" || arg == "-r" {
            if let Some(arg) = std::env::args().nth(count + 1) {
                //get the next argument and see if it is = to gogo of vrv
                if arg == "zoro" {
                    provider = "zoro".to_string();
                    count += 1;
                } else if arg == "gogo" {
                    provider = "gogoanime".to_string();
                    count += 1;
                } else {
                    provider = "zoro".to_string();
                }
            } else {
                provider = "zoro".to_string();
            }
        }
        if arg == "--cast" || arg == "-C" {
            if let Some(arg) = std::env::args().nth(count + 1) {
                cast = (true, String::from(arg))
            } else {
                println!("{}", "please provide a ip address".red())
            }
        }

        if arg == "--ln" || arg == "-l" {
            ln = true;
        }
        if arg == "--chapter" || arg == "-c" {
            if let Some(arg) = std::env::args().nth(count + 1) {
                chapter = arg.parse::<u32>().unwrap();
            } else {
                chapter = 0;
            }
        }

        count += 1;
    }

    if help == true {
        print_help();
    }
    if anime == false && ln == false {
        println!("1:    Anime");
        println!("2:    Light Novel");

        let a = int_input("pick your poison: ");
        match a {
            1 => anime = true,
            2 => ln = true,
            _ => println!("invalid option. "),
        };
    }
    if anime == true && ln == true {
        println!("you can only use one of the arguments at a time");
        std::process::exit(0);
    }
    if ln == true {
        //ln_read(&search, chapter);
        _ = ln_ui(chapter);
    } else if anime == true {
        //anime_stream(search, episode, resume);

        let token = get_token();
        _ = anime_ui(token, provider, cast);
    } else {
        println!("Invalid argument");
    }
}

fn print_help() {
    println!("anime:\t\t{}", format_args!("{}", "-a --anime".red()));
    //print blank line
    println!("");
    println!(
        "cast:\t\t{}",
        format_args!("{} {}", "-C --cast".red(), "<IP Adress>".green())
    );
    println!("");
    println!("light novel:\t{}", format_args!("{}", "-l --ln".red()));
    //print blank line
    println!("");
    println!("chapter:\t{}", format_args!("{}", "-c --chapter".red()));
    println!(
        "{}",
        "after this^^^ argument you can enter a chapter number".green()
    );
    println!("{}", "for exaple kami -c 200");
    //print blank line
    println!("");
    println!("provider:\t{}", format_args!("{}", "-r --provider".red()));
    println!(
        "{}",
        "after this^^^ argument you can enter a provider".green()
    );
    println!(
        "if no provider is entered it will default to {}",
        "gogo".green()
    );
    println!(
        "if the -r argument is not used it will default to {}",
        "zoro".green()
    );
    println!("the providers are {} or {}", "gogo".green(), "zoro".green());
    println!("");
    println!("help:\t\t{}", format_args!("{}", "-h --help".red()));
    //kill the program
    std::process::exit(0);
}
