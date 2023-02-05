use isahc::config::Configurable;
use isahc::{ReadResponseExt, Request, RequestExt};
use std::fs::File;
use std::io::prelude::*;

use crate::helpers::{fixing_text::htmlise, scraper};

//use serde_json::json;

pub fn search_anime(query: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let req = Request::builder()
        .uri(format!(
            "https://api.consumet.org/meta/anilist/{}",
            htmlise(query)
        ))
        .redirect_policy(isahc::config::RedirectPolicy::Follow)
        .header("user-agent", *scraper::USER_AGENT)
        .body(())
        .unwrap();
    let json = req.send().unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();

    let mut titles = Vec::new();
    let mut ids = Vec::new();
    let mut images = Vec::new();

    for anime in json["results"].as_array().unwrap().iter() {
        titles.push(
            anime["title"]["userPreferred"]
                .as_str()
                .unwrap()
                .to_string(),
        );

        ids.push(anime["id"].as_str().unwrap().to_string());
        images.push(anime["image"].as_str().unwrap().to_string());
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
        .header("user-agent", *scraper::USER_AGENT)
        .body(())
        .unwrap();
    let json = req.send().unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();

    let mut titles = Vec::new();
    let mut ids = Vec::new();

    for episode in json["episodes"].as_array().unwrap().iter() {
        titles.push(episode["title"].as_str().unwrap().to_string());
        ids.push(episode["id"].as_str().unwrap().to_string());
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
        .header("user-agent", *scraper::USER_AGENT)
        .body(())
        .unwrap();
    let json = req.send().unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();

    let mut url = String::new();
    std::fs::write("test.json", json.to_string()).unwrap();

    let mut subtitle = String::new();
    let _error_vec = Vec::new();

    let sub_array = json["subtitles"].as_array().unwrap_or(&_error_vec);

    for sub in sub_array.iter() {
        //set subtitle to lang = English
        if sub["lang"].as_str().unwrap_or("null") == "English" {
            subtitle = sub["url"].as_str().unwrap_or("null").to_string();
            // add \ before the first : in the url
            subtitle = subtitle.replace(":", "\\:");
        }
    }
    let mut highest_quality = 0;
    for source in json["sources"].as_array().unwrap().iter() {
        let quality = source["quality"]
            .as_str()
            .unwrap()
            .replace("p", "")
            .parse::<i32>()
            .unwrap_or(0);
        if quality > highest_quality {
            highest_quality = quality;
            url = source["url"].as_str().unwrap().to_string();
        }
    }
    (url, subtitle)
}

pub fn get_image(url: &str, path: &str) {
    let url = url;
    let mut response = isahc::get(url).unwrap();
    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&buffer).unwrap();
}
