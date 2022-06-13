use regex::Regex;
use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use colored::Colorize;
use isahc::prelude::*;
use std::process::{Command, ExitStatus};
use std::io::Result;
fn main() {
    let ln_url = search_ln();
    let mut selected_page = 1;
    loop {
        let chapter_url = chapter_selector(&ln_url, selected_page);
        selected_page = chapter_url.1;
        let full_text = get_full_text(&chapter_url.0);
        //write full_text to file called temp.txt
        let mut file = File::create("/tmp/log_e").expect("Unable to create file");
        file.write_all(full_text.as_bytes()).expect("Unable to write to file");
        //close file
        file.sync_all().expect("Unable to sync file");
        //open temp.txt in cat for user to read
        let _com = open_bat();
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn search_ln()->String{
    let mut _is_n = false;
    while _is_n == false{
        let mut search_path = String::new();
        println!("What ln do you want to read?");
        std::io::stdin().read_line(&mut search_path).expect("Failed to read line");
        let search_path = search_path.replace(" ", "+");
        let url = "https://readlightnovels.net/?s=".to_string();
        let url = format!("{}{}", url, search_path.trim()).trim().to_string();
        let html = get_html(&url).trim().to_string();
        let ln_list = get_ln_list(&html);
        //remove first element of ln_list
        let ln_list = ln_list.iter().skip(1).map(|x| x.to_string()).collect::<Vec<String>>();
        let ln_titles = get_ln_titles(&ln_list);
        let ln_urls = get_ln_urls(&ln_list);
        let mut count = 0;
        for ln in ln_titles {
            if count % 2 == 0 {
                println!("({})\t{}",count, format!("{}", ln.blue()));
            } else {
                println!("({})\t{}",count, format!("{}", ln.yellow()));
            }
            count += 1;
        }
        println!("(n)\t{}","Search another title".red());
        let mut ln_number = String::new();
        std::io::stdin().read_line(&mut ln_number).expect("Failed to read line");
        ln_number = ln_number.trim().to_string();
        if ln_number != "n" {
            let ln_number = ln_number.trim().to_string();
            let ln_number = ln_number.parse::<usize>().unwrap();
            let ln_url = &ln_urls[ln_number];
            let ln_url = ln_url.trim().to_string();
            _is_n = true;
            return ln_url;
        }
        print!("\x1B[2J\x1B[1;1H");
    }
    return "".to_string();
}

fn chapter_selector(ln_url: &String, mut selected_page: u32)->(String, u32){
    let exit = false;
    while exit == false{
        let ln_html = get_html(&ln_url);
        let ln_id = get_ln_id(&ln_html);
        let ln_last_page = get_ln_last_page(&ln_html);
        let ln_page_html = page_selector(&ln_id,selected_page);
        let ln_chapters = get_ln_chapters(&ln_page_html);
        let ln_chapters_urls = get_ln_chapters_urls(&ln_page_html);
        let mut count = 0;
        for chaprer in ln_chapters {
            if count % 2 == 0 {
                println!("({})\t{}",count, format!("{}", chaprer.blue()));
            } else {
                println!("({})\t{}",count, format!("{}", chaprer.yellow()));
            }
            count += 1;
        }
        println!("(n)\t{}","Go to next page".red());
        println!("(b)\t{}","Go to previous page".red());
        println!("Which chapter do you want to read?");
        let mut chapter_number = String::new();
        std::io::stdin().read_line(&mut chapter_number).expect("Failed to read line");
        chapter_number = chapter_number.trim().to_string();
        if chapter_number == "n" && selected_page < ln_last_page.parse::<u32>().unwrap() {
            selected_page += 1;
            print!("\x1B[2J\x1B[1;1H");
        }
        else if chapter_number == "b" && selected_page > 1{
            selected_page -= 1;
            print!("\x1B[2J\x1B[1;1H");
        }
        else{
            let chaprer_number = chapter_number.trim().to_string();
            let chaprer_number = chaprer_number.parse::<usize>().unwrap();
            let chaprer_url = &ln_chapters_urls[chaprer_number];
            let chaprer_url = chaprer_url.trim().to_string();
            return (chaprer_url, selected_page);
        }
    }
    return ("".to_string(),1);
}

fn get_full_text(chapter_url: &String)->String{
    let ln_text = get_ln_text(&chapter_url);
    let mut full_text: String = String::new();
    for line in ln_text {
        let text = format!("{}\n\n", line);
        full_text.push_str(&text);
    }
    full_text
}

pub fn open_bat() -> Result<ExitStatus> {
    Command::new("bat").arg("-p").arg("/tmp/log_e").spawn()?.wait()
}

//gets the full html of the page
fn get_html(url: &str) -> String {
    let mut resp = isahc::get(url).unwrap();
    let html = resp.text().unwrap();
    html
}
//gets the list of ln's from the html and returns it as a vector of the ln's name and href
fn get_ln_list(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?m)^\s*(<a href="[^"]*" title="[^"]*")"#).unwrap();
    let mut ln_list:Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list
}
//gets the titles of the ln's from the html and returns it as a vector of the ln's name
fn get_ln_titles(ln_list: &Vec<String>) -> Vec<String> {
    let re = Regex::new(r#"(?m)^\s*<a href="[^"]*" title="([^"]*)""#).unwrap();
    let mut ln_title:Vec<String> = Vec::new();
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
    let mut ln_url:Vec<String> = Vec::new();
    for ln in ln_list {
        for cap in re.captures_iter(ln) {
            ln_url.push(cap.get(1).unwrap().as_str().to_string());
        }
    }
    ln_url
}

//gets the chapter titles from the html and returns it as a vector of the chapter's name
fn get_ln_chapters(html: &str) -> Vec<String> {
    let re = Regex::new(r#"title=(.*?)>"#).unwrap();
    let mut ln_list:Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list = remove_after_dash(&ln_list);
    ln_list
}


//function that takes a vector called ln_chapters of strings and removes everyting after the first occurence of "-" and all \ and "
fn remove_after_dash(ln_chapters: &Vec<String>) -> Vec<String> {
    let mut ln_chapters_new:Vec<String> = Vec::new();
    let re = Regex::new(r#"\\"(.*?) -"#).unwrap();
    for ln in ln_chapters {
        for cap in re.captures_iter(ln) {
            ln_chapters_new.push(cap.get(1).unwrap().as_str().trim().to_string());
        }
    }
    ln_chapters_new = replace_unicode(&ln_chapters_new);
    ln_chapters_new
}

//function that takes a vector called ln_chapters and looks for unicode characters and replaces them with the ascii version
fn replace_unicode(ln_chapters: &Vec<String>) -> Vec<String> {
    let mut ln_chapters_new:Vec<String> = Vec::new();
    for ln in ln_chapters {
        //make regex to find all \uxxxx and save it in to a vector
        let re = Regex::new(r#"(\\u[0-9a-fA-F]{4})"#).unwrap();
        let mut vec_unicode:Vec<String> = Vec::new();
        for cap in re.captures_iter(ln) {
            vec_unicode.push(cap.get(1).unwrap().as_str().to_string());
        }
        let mut ln_new :String = String::new();
        if vec_unicode.len() > 0 {
            //loop through the vector and replace the unicode characters with the ascii version
            for unicode in vec_unicode {
                //convert the unicode to char
                let unicode_char = char::from_u32(u32::from_str_radix(&unicode[2..6], 16).unwrap()).unwrap();
                let unicode_str = unicode_char as char;
                ln_new = ln.replace(&unicode, &unicode_str.to_string());
            }
        }
        else {
            ln_new = ln.to_string();
        }
       ln_chapters_new.push(ln_new);
    }
    ln_chapters_new
}

fn fix_html_encoding(ln_text: &Vec<String>) -> Vec<String>{
    let mut ln_text_new:Vec<String> = Vec::new();
    for ln in ln_text{
        let ln = ln.replace("&#8213;", "--");
        let ln = ln.replace("&#8214;", "--");
        let ln = ln.replace("&#8216;", "'");
        let ln = ln.replace("&#8217;", "'");
        let ln = ln.replace("&#8220;", "\"");
        let ln = ln.replace("&#8221;", "\"");
        let ln = ln.replace("&#8230;", "...");
        let ln = ln.replace("&#8242;", "'");
        let ln = ln.replace("&#8243;", "\"");
        let ln = ln.replace("&#8260;", "--");
        let ln = ln.replace("&#8212;", "--");
        ln_text_new.push(ln);

    }
    ln_text_new
}
//gets the chapter urls from the html and returns it as a vector of the chapter's href
fn get_ln_chapters_urls(html: &str) -> Vec<String> {
    let re = Regex::new(r#"href=\\"(h.*?)""#).unwrap();
    let mut ln_list:Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list = url_clean(&ln_list);
    ln_list
}// take a vector of srings called ln_chapters_url and remove all the \
fn url_clean(ln_chapters_url: &Vec<String>) -> Vec<String> {
    let mut ln_chapters_url_new:Vec<String> = Vec::new();
    for ln in ln_chapters_url {
        let ln = ln.replace("\\", "");
        ln_chapters_url_new.push(ln);
    }
    ln_chapters_url_new
}

//take a html string and return the ln id
fn get_ln_last_page(html: &str) -> String {
    let re = Regex::new(r#"(?m)<a data-page="([0-9]+)" href="[^"]*" title="[0-9]+">Last</a>"#).unwrap();
    let mut ln_last_page:String = String::new();
    for cap in re.captures_iter(html) {
        ln_last_page = cap.get(1).unwrap().as_str().to_string();
    }
    ln_last_page
}

//take a html string and return the ln id
fn get_ln_id(html: &str) -> String {
    let re = Regex::new(r#"(?m)^\s*<input id="id_post" type="hidden" value="([0-9]+)">"#).unwrap();
    let mut ln_id:String = String::new();
    for cap in re.captures_iter(html) {
        ln_id = cap.get(1).unwrap().as_str().to_string();
    }
    ln_id
}

//using isahc::prelude::* make a php reqest to get the next page of the ln
fn get_ln_next_page(ln_id: &str, page: &str) -> String {
    let url = "https://readlightnovels.net/wp-admin/admin-ajax.php".to_string();
    let form = format!("action=tw_ajax&type=pagination&id={}.html&page={}", ln_id, page);
    let mut resp = isahc::post(url,form).unwrap();
    let html = resp.text().unwrap();
    html
}

fn page_selector(ln_id: &str,selected_page: u32) -> String {

    get_ln_next_page(&ln_id, &selected_page.to_string())
}

fn get_ln_text(chapter_url: &str) -> Vec<String> {
    let mut resp = isahc::get(chapter_url).unwrap();
    let html = String::from(resp.text().unwrap());
    let re = Regex::new(r#"(?m)<p>(.*?)</p>"#).unwrap();
    let mut ln_text:Vec<String> = Vec::new();
    for cap in re.captures_iter(&html) {
        ln_text.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    // remove last 3 indexes of ln_text
    ln_text.truncate(ln_text.len() - 3);
    let ln_text = fix_html_encoding(&ln_text);
    ln_text
}
