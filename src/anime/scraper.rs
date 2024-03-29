use isahc::config::Configurable;
use isahc::{ReadResponseExt, Request, RequestExt};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

//use serde_json::json;

pub fn get_anime_html(url: &str) -> String {
    let req = Request::builder()
        .uri(url)
        .redirect_policy(isahc::config::RedirectPolicy::Follow)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(())
        .unwrap();
    req.send().unwrap().text().unwrap()
}

pub fn get_post(id: &str) -> String {
    let resp = Request::builder()
        .method("POST")
        .uri("https://yugenanime.ro/api/embed/")
        .header("x-requested-with", "XMLHttpRequest")
        .body(id)
        .unwrap()
        .send()
        .unwrap()
        .text();
    let resp: String = resp.as_ref().unwrap().to_string();
    resp
}

pub fn get_animes(query: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let query = query.replace(" ", "+");
    let html = get_anime_html(&format!("https://yugenanime.ro/discover/?q={}", query));
    let re = Regex::new(r#"href="(/anime[^"]*)""#).unwrap();
    let mut animes_links = Vec::new();
    for cap in re.captures_iter(&html) {
        animes_links.push(cap[1].to_string());
    }
    let re = Regex::new(r#"/" title="([^"]*)""#).unwrap();
    let mut animes_names = Vec::new();
    for cap in re.captures_iter(&html) {
        animes_names.push(cap[1].to_string());
    }
    let re = Regex::new(r#"data-src="([^"]*)"#).unwrap();
    let mut animes_images = Vec::new();
    for cap in re.captures_iter(&html) {
        animes_images.push(cap[1].to_string());
    }
    (animes_links, animes_names, animes_images)
}

pub fn get_anime_info(url: &str) -> (i32, u16) {
    let url = format!("https://yugenanime.ro{}watch", url);
    let html = get_anime_html(&url);
    //print html and exit
    let re = Regex::new(r#""mal_id":(\d*)"#).unwrap();
    let mal_id = re.captures(&html).unwrap()[1].parse().unwrap();
    let re =
        Regex::new(r#"Episodes</div><span class="description" style="font-size: \d*px;">(\d*)"#)
            .unwrap();
    let episodes = re.captures(&html).unwrap()[1].parse().unwrap();
    (mal_id, episodes)
}

pub fn get_anime_link(url: &str, episode: u64) -> String {
    let url = &format!(
        "https://yugenanime.ro/watch{}{}/",
        url.replace("/anime", ""),
        episode
    );
    let html = get_anime_html(url);
    let re = Regex::new(r#"/e/([^/]*)"#).unwrap();
    let capture = re.captures(&html).unwrap();
    let id = &capture[1];
    let id = format!("id={}%3D&ac=0", id);
    let json = get_post(&id);
    let re = Regex::new(r#"hls": \["(.*)","#).unwrap();
    let capture = re.captures(&json).unwrap();
    let link = &capture[1];
    //return the link
    link.to_string()
}

pub fn get_image(url: &str, path: &str) {
    let url = url;
    let mut response = isahc::get(url).unwrap();
    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&buffer).unwrap();
}
