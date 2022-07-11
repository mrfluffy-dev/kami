use crate::helpers::{fixing_text::remove_after_dash, take_input::string_input};
use colored::Colorize;
use regex::Regex;

pub fn search_ln() -> String {
    let mut _is_n = false;
    print!("\x1B[2J\x1B[1;1H");
    while !_is_n {
        let search_path = string_input("What ln do you want to read? ");
        let search_path = search_path.replace(' ', "+");
        let url = "https://readlightnovels.net/?s=".to_string();
        let url = format!("{}{}", url, search_path.trim()).trim().to_string();
        let html = crate::ln::scraper::get_html(&url).trim().to_string();
        let ln_list = get_ln_list(&html);
        //remove first element of ln_list
        let ln_list = ln_list
            .iter()
            .skip(1)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let ln_titles = get_ln_titles(&ln_list);
        let ln_urls = get_ln_urls(&ln_list);
        let mut count = 0;
        ln_titles.into_iter().for_each(|ln| {
            if count % 2 == 0 {
                println!("({})\t{}", count, format_args!("{}", ln.blue()));
            } else {
                println!("({})\t{}", count, format_args!("{}", ln.yellow()));
            }
            count += 1;
        });
        println!("(s)\t{}", "Search another title".green());
        let ln_number = string_input("Enter an option: ");
        if ln_number != "s" && ln_number.parse::<usize>().is_ok() {
            let ln_number = ln_number.trim().to_string();
            let ln_number = ln_number.parse::<usize>().unwrap();
            let ln_url = &ln_urls[ln_number];
            let ln_url = ln_url.trim().to_string();
            _is_n = true;
            print!("\x1B[2J\x1B[1;1H");
            return ln_url;
        } else {
            print!("invalid input");
        }
        print!("\x1B[2J\x1B[1;1H");
    }
    "".to_string()
}

//gets the list of ln's from the html and returns it as a vector of the ln's name and href
fn get_ln_list(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?m)^\s*(<a href="[^"]*" title="[^"]*")"#).unwrap();
    let mut ln_list: Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list
}
//gets the titles of the ln's from the html and returns it as a vector of the ln's name
fn get_ln_titles(ln_list: &Vec<String>) -> Vec<String> {
    let re = Regex::new(r#"(?m)^\s*<a href="[^"]*" title="([^"]*)""#).unwrap();
    let mut ln_title: Vec<String> = Vec::new();
    for ln in ln_list {
        for cap in re.captures_iter(ln) {
            ln_title.push(cap.get(1).unwrap().as_str().to_string());
        }
    }
    ln_title
}

//gets the urls of the ln's from the html and returns it as a vector of the ln's href
fn get_ln_urls(ln_list: &Vec<String>) -> Vec<String> {
    let re = Regex::new(r#"(?m)^\s*<a href="([^"]*)""#).unwrap();
    let mut ln_url: Vec<String> = Vec::new();
    for ln in ln_list {
        for cap in re.captures_iter(ln) {
            ln_url.push(cap.get(1).unwrap().as_str().to_string());
        }
    }
    ln_url
}

//gets the chapter titles from the html and returns it as a vector of the chapter's name
pub fn get_ln_chapters(html: &str) -> Vec<String> {
    let re = Regex::new(r#"title=(.*?)>"#).unwrap();
    let mut ln_list: Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list = remove_after_dash(&ln_list);
    ln_list
}
