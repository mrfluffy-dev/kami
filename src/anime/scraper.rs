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

pub fn get_episodes(id: &i32) -> (Vec<String>, Vec<String>) {
    let req = Request::builder()
        .uri(format!(
            "https://api.consumet.org/meta/anilist/info/{}?provider=gogoanime",
            id
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

pub fn get_episode_link(ep_id: &str) -> String {
    let req = Request::builder()
        .uri(format!(
            "https://api.consumet.org/meta/anilist/watch/{}?provider=gogoanime",
            ep_id
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
    let url = "";
    std::fs::write("test.json", json.to_string()).unwrap();
    for i in 0..json["sources"].as_array().unwrap().len() {
        //return json["sources"][i]["url"].as_str().unwrap().to_string(); where json["sources"][i]["quality"].as_str().unwrap().contains("1080")
        if json["sources"][i]["quality"]
            .as_str()
            .unwrap()
            .contains("1080")
        {
            return json["sources"][i]["url"].as_str().unwrap().to_string();
        }
    }
    url.to_string()
}

pub fn get_image(url: &str, path: &str) {
    let url = url;
    let mut response = isahc::get(url).unwrap();
    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&buffer).unwrap();
}
