mod anime;
mod helpers;
mod ln;

use std::fs::File;
use std::io::Write;

use colored::Colorize;
use ln::scraper::get_ln_next_page;
use ln::search::search_ln;

use crate::anime::{
    player::open_video,
    scraper::{anime_ep_range, anime_link, anime_names},
};
use crate::helpers::take_input::{int_input, string_input, u16_input};
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
        let ln_url = search_ln();
        let mut selected_page = 1;
        loop {
            //make empty tuple called chapter_url with (String, u32, u32)
            let chapter_url = chapter_selector(&ln_url, selected_page);
            selected_page = chapter_url.1;
            let full_text = get_full_text(&chapter_url.0);
            //write full_text to file called temp.txt
            let mut file = File::create("/tmp/log_e").expect("Unable to create file");
            file.write_all(full_text.as_bytes())
                .expect("Unable to write to file");
            //close file
            file.sync_all().expect("Unable to sync file");
            //open temp.txt in cat for user to read
            let _com = open_bat();
            print!("\x1B[2J\x1B[1;1H");
        }
    } else if _arg == "a" {
        let query = if std::env::args().len() > 2 {
            std::env::args().nth(2).unwrap()
        } else {
            string_input("Enter query: ")
        };
        let anime_list = anime_names(&query);
        let mut count = 0;
        print!("\x1B[2J\x1B[1;1H");
        anime_list.iter().for_each(|anime| {
            if count % 2 == 0 {
                println!(
                    "({})\t{}",
                    format_args!("{}", count.to_string().blue()),
                    format_args!("{}", anime.blue())
                );
            } else {
                println!(
                    "({})\t{}",
                    format_args!("{}", count.to_string().yellow()),
                    format_args!("{}", anime.yellow())
                );
            }
            count += 1;
        });
        let anime_num = int_input("Enter anime number: ");
        let title = &anime_list[anime_num];
        let ep_range = anime_ep_range(title);
        // if there is only one episode, then don't ask user to choose episode
        if ep_range == 1 {
            let link = anime_link(title, 1);
            open_video(link);
            main();
        } else {
            println!("select episode 1-{}: ", ep_range);
            let ep_num = u16_input("Enter episode number: ");
            let link = anime_link(title, ep_num);
            open_video(link);
            main();
        }
    } else {
        println!("Invalid argument");
    }
}

fn page_selector(ln_id: &str, selected_page: u32) -> String {
    get_ln_next_page(ln_id, &selected_page.to_string())
}
