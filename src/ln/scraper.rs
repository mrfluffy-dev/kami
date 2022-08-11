use isahc::{ReadResponseExt, Request, RequestExt};
use regex::Regex;

use crate::helpers::fixing_text::remove_after_dash;

use crate::helpers::fixing_text::fix_html_encoding;

//gets the full html of the page
pub fn get_html(url: &str) -> String {
    let req = Request::builder()
        .uri(url)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(())
        .unwrap();

    req.send().unwrap().text().unwrap()
}

//using isahc::prelude::* make a php reqest to get the next page of the ln
pub fn get_ln_next_page(ln_id: &str, page: &str) -> String {
    let url = "https://readlightnovels.net/wp-admin/admin-ajax.php".to_string();
    let form = format!(
        "action=tw_ajax&type=pagination&id={}.html&page={}",
        ln_id, page
    );
    //let mut resp = isahc::post(&url,form).unwrap();
    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(form)
        .unwrap();

    req.send().unwrap().text().unwrap()
}

pub fn get_full_text(chapter_url: &str) -> String {
    let ln_text = get_ln_text(chapter_url);
    let mut full_text: String = String::new();
    for line in ln_text {
        let text = format!("{}\n\n", line);
        full_text.push_str(&text);
    }
    full_text
}

//gets the chapter urls from the html and returns it as a vector of the chapter's href
pub fn get_ln_chapters_urls(html: &str) -> Vec<String> {
    let re = Regex::new(r#"href=\\"(h.*?)""#).unwrap();
    let mut ln_list: Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list = url_clean(&ln_list);
    ln_list
} // take a vector of srings called ln_chapters_url and remove all the \

pub fn url_clean(ln_chapters_url: &Vec<String>) -> Vec<String> {
    let mut ln_chapters_url_new: Vec<String> = Vec::new();
    for ln in ln_chapters_url {
        let ln = ln.replace('\\', "");
        ln_chapters_url_new.push(ln);
    }
    ln_chapters_url_new
}

//take a html string and return the ln id
pub fn get_ln_last_page(html: &str) -> String {
    let re =
        Regex::new(r#"(?m)<a data-page="([0-9]+)" href="[^"]*" title="[0-9]+">Last</a>"#).unwrap();
    let mut ln_last_page: String = String::new();
    for cap in re.captures_iter(html) {
        ln_last_page = cap.get(1).unwrap().as_str().to_string();
    }
    ln_last_page
}

//take a html string and return the ln id
pub fn get_ln_id(html: &str) -> String {
    let re = Regex::new(r#"(?m)^\s*<input id="id_post" type="hidden" value="([0-9]+)">"#).unwrap();
    let mut ln_id: String = String::new();
    for cap in re.captures_iter(html) {
        ln_id = cap.get(1).unwrap().as_str().to_string();
    }
    ln_id
}

pub fn get_ln_text(chapter_url: &str) -> Vec<String> {
    let mut resp = isahc::get(chapter_url).unwrap();
    let html = resp.text().unwrap();
    let re = Regex::new(r#"(?m)<p>(.*?)</p>"#).unwrap();
    let mut ln_text: Vec<String> = Vec::new();
    for cap in re.captures_iter(&html) {
        ln_text.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    // remove last 3 indexes of ln_text
    ln_text.truncate(ln_text.len() - 3);

    fix_html_encoding(&ln_text)
}

//gets the list of ln's from the html and returns it as a vector of the ln's name and href
pub fn get_ln_list(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?m)^\s*(<a href="[^"]*" title="[^"]*")"#).unwrap();
    let mut ln_list: Vec<String> = Vec::new();
    for cap in re.captures_iter(html) {
        ln_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    ln_list
}
//gets the titles of the ln's from the html and returns it as a vector of the ln's name
pub fn get_ln_titles(ln_list: &Vec<String>) -> Vec<String> {
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
pub fn get_ln_urls(ln_list: &Vec<String>) -> Vec<String> {
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
