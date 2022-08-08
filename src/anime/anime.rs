use crate::main;
use crate::open_video;
use crate::{anime_ep_range, anime_link, anime_names};
use crate::{get_anime_id, get_token, get_user_anime_progress, update_anime_progress};
use crate::{int_input, string_input};
use colored::Colorize;
//use crate
pub fn anime_stream(search: String, episode: u32) {
    let token = get_token();
    let query = if search != "" {
        search
    } else {
        string_input("Search anime: ")
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
    let mut anime_num: usize = usize::MAX;
    while anime_num == usize::max_value() || anime_num > anime_list.len() {
        anime_num = int_input("Enter anime number: ");
        if anime_num > anime_list.len() {
            println!("Invalid anime number");
        }
    }
    let title = &anime_list[anime_num];
    let ep_range = anime_ep_range(title);
    // if there is only one episode, then don't ask user to choose episode
    let id = get_anime_id(&title.replace("-", " "));
    if ep_range == 1 {
        let link = anime_link(title, 1);
        open_video(link);
        update_anime_progress(id, &title.replace("-", " "), 1, &token);
        main();
    } else {
        let mut ep_num: usize = usize::MAX;
        if episode > ep_range.into() {
            println!("Invalid episode number");
            main();
        } else if episode != 0 {
            ep_num = episode as usize;
        } else {
            let current_progress = get_user_anime_progress(id, &token);
            println!("you are currently on episode: {}", current_progress);
            println!("select episode 1-{}: ", ep_range);
            while ep_num == usize::max_value() || ep_num > ep_range as usize {
                ep_num = int_input("Enter episode number: ");
                if ep_num > ep_range as usize {
                    println!("Invalid episode number");
                }
            }
        }
        loop {
            let link = anime_link(title, ep_num as u64);
            open_video(link);
            let id = get_anime_id(&title.replace("-", " "));
            println!("{}", get_user_anime_progress(id, &token));
            update_anime_progress(id, &title.replace("-", " "), ep_num, &token);
            println!("{}", "n: next episode".green());
            println!("{}", "p: previous episode".yellow());
            println!("{}", "s: search another anime".green());
            println!("{}", "q: quit".red());
            let input = string_input("Enter command: ");
            if input == "n" {
                if ep_num == ep_range as usize {
                    println!("No more episodes");
                } else {
                    ep_num += 1;
                }
            } else if input == "p" {
                if ep_num == 1 {
                    println!("No previous episodes");
                } else {
                    ep_num -= 1;
                }
            } else if input == "s" {
                //remove all the arguments
                anime_stream("".to_string(), 0);
            } else if input == "q" {
                std::process::exit(0);
            } else {
                println!("Invalid command");
            }
        }
    }
}
