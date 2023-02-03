use isahc::config::Configurable;
use isahc::{ReadResponseExt, Request, RequestExt};
use std::fs::File;
use std::io::prelude::*;

//use serde_json::json;

pub fn search_anime(query: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let req = Request::builder()
        .uri(format!(
            "https://api.consumet.org/meta/anilist/{}",
            query
                .replace(" ", "%20")
                .replace(":", "%3A")
                .replace("!", "%21")
                .replace("?", "%3F")
                .replace("'", "%27")
        ))
        .redirect_policy(isahc::config::RedirectPolicy::Follow)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(())
        .unwrap();
    let json = req.send().unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let mut titles = Vec::new();
    let mut ids = Vec::new();
    let mut images = Vec::new();
    for i in 0..json["results"].as_array().unwrap().len() {
        titles.push(
            json["results"][i]["title"]["userPreferred"]
                .as_str()
                .unwrap()
                .to_string(),
        );

        ids.push(json["results"][i]["id"].as_str().unwrap().to_string());
        //convert ids to i32
        images.push(json["results"][i]["image"].as_str().unwrap().to_string());
    }
    (ids, titles, images)
}

pub fn get_episodes(id: &i32, provider: &str) -> (Vec<String>, Vec<String>) {
    let req = Request::builder()
        .uri(format!(
            "https://api.consumet.org/meta/anilist/info/{}?provider={}",
            id, provider
        ))
        .redirect_policy(isahc::config::RedirectPolicy::Follow)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(())
        .unwrap();
    let json = req.send().unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let mut titles = Vec::new();
    let mut ids = Vec::new();
    for i in 0..json["episodes"].as_array().unwrap().len() {
        titles.push(json["episodes"][i]["title"].as_str().unwrap().to_string());
        ids.push(json["episodes"][i]["id"].as_str().unwrap().to_string());
    }
    (titles, ids)
}

pub fn get_episode_link(ep_id: &str, provider: &str) -> (String, String) {
    let req = Request::builder()
        .uri(format!(
            "https://api.consumet.org/meta/anilist/watch/{}?provider={}",
            ep_id, provider
        ))
        .redirect_policy(isahc::config::RedirectPolicy::Follow)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(())
        .unwrap();
    let json = req.send().unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let mut url = String::new();
    std::fs::write("test.json", json.to_string()).unwrap();
    let mut subtitle = String::new();
    let _error_vec = Vec::new();
    let sub_array = json["subtitles"].as_array().unwrap_or(&_error_vec);
    for i in 0..sub_array.len() {
        //set subtitle to lang = English
        if json["subtitles"][i]["lang"].as_str().unwrap_or("null") == "English" {
            subtitle = json["subtitles"][i]["url"]
                .as_str()
                .unwrap_or("null")
                .to_string();
            // add \ before the first : in the url
            subtitle = subtitle.replace(":", "\\:");
        }
    }
    let mut highest_quality = 0;
    for i in 0..json["sources"].as_array().unwrap().len() {
        let quality = json["sources"][i]["quality"]
            .as_str()
            .unwrap()
            .replace("p", "")
            .parse::<i32>()
            .unwrap_or(0);
        if quality > highest_quality {
            highest_quality = quality;
            url = json["sources"][i]["url"].as_str().unwrap().to_string();
        }
    }
    (url.to_string(), subtitle)
}

pub fn get_image(url: &str, path: &str) {
    let url = url;
    let mut response = isahc::get(url).unwrap();
    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&buffer).unwrap();
}
