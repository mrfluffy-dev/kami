use crate::{string_input,int_input};
use crate::{anime_names,anime_ep_range,anime_link};
use crate::open_video;
use crate::main;
use colored::Colorize;
//use crate
pub fn anime_stream(){
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
    if ep_range == 1 {
        let link = anime_link(title, 1);
        open_video(link);
        main();
    } else {
        loop{
            println!("select episode 1-{}: ", ep_range);
            let mut ep_num: usize = usize::MAX;
            while ep_num == usize::max_value() || ep_num > ep_range as usize {
                ep_num = int_input("Enter episode number: ");
                if ep_num > ep_range as usize {
                    println!("Invalid episode number");
                }
            }
            let link = anime_link(title, ep_num as u64);
            open_video(link);
        }
    }
}
