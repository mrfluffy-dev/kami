use crate::string_input;
use isahc::{ReadResponseExt, Request, RequestExt};
use serde_json::json;
use std::fs;

pub fn get_token() -> String {
    //if not on windows create folder ~/.config/kami
    let config_path = dirs::config_dir().unwrap().join("kami");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }
    let token_path = config_path.join("token.txt");
    if !token_path.exists() {
        //create empty file
        fs::File::create(&token_path).unwrap();
    }
    //read token from file
    let token = fs::read_to_string(&token_path).unwrap();
    if token.is_empty() {
        //ask user if they want to add a token or track locally
        let input = string_input(
            "would you want to link anilist(sellecting no will track anime localy)? (y/n)",
        );
        if input == "y" {
            println!("please go to the below link and copy and past the token below");
            println!(
                "https://anilist.co/api/v2/oauth/authorize?client_id=9121&response_type=token"
            );
            let token = string_input("token: ");
            fs::write(&token_path, token).unwrap();
        } else if input == "n" {
            let token = "local";
            fs::write(&token_path, token).unwrap();
        } else {
            println!("invalid input");
            std::process::exit(1);
        }
    }
    let token = fs::read_to_string(&token_path).unwrap();
    token
}

//get the user id from the token
fn get_user_id(token: &str) -> i32 {
    const QUERY: &str = "query {
    Viewer {
        id
    }
}";
    let json = json!({ "query": QUERY });
    let resp = Request::builder()
        .method("POST")
        .uri("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(json.to_string())
        .unwrap()
        .send()
        .unwrap()
        .text();
    //println!("{}", resp);
    let regex = regex::Regex::new(r#"id":(.*?)}"#).unwrap();
    let resp: String = resp.as_ref().unwrap().to_string();
    let id = regex
        .captures(&resp)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    id
}

pub fn get_user_anime_progress(anime_id: i32, token: &str) -> i32 {
    let user_id = get_user_id(&token);
    const QUERY: &str = "query ($user_id: Int, $media_id: Int) {
    MediaList (userId: $user_id, mediaId: $media_id, type: ANIME) {
        progress
    }
}";
    let json = json!({
        "query": QUERY,
        "variables": {
            "user_id": user_id,
            "media_id": anime_id,
        }
    });
    let resp = Request::builder()
        .method("POST")
        .uri("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(json.to_string())
        .unwrap()
        .send()
        .unwrap()
        .text();
    let regex = regex::Regex::new(r#"progress":(.*?)}"#).unwrap();
    let resp: String = resp.as_ref().unwrap().to_string();
    if resp.contains("errors") {
        0
    } else {
        let progress = regex
            .captures(&resp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        progress
    }
}

pub fn update_anime_progress(anime_id: i32, progress: usize, token: &str) {
    const UPDATE: &str = "
mutation ($mediaId: Int, $status: MediaListStatus, $progress: Int) {
    SaveMediaListEntry (mediaId: $mediaId, status: $status, progress: $progress) {
        id
        status
        progress
    }
}
";
    let json = json!({
        "query": UPDATE,
        "variables": {
            "mediaId": anime_id,
            "status": "CURRENT",
            "progress": progress
        }
    });
    let _resp = Request::builder()
        .method("POST")
        .uri("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(json.to_string())
        .unwrap()
        .send()
        .unwrap()
        .text();
}

// local tracking
pub fn get_an_json() -> serde_json::Value {
    let config_path = dirs::config_dir().unwrap().join("kami");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }
    let json_path = config_path.join("an_progress.json");
    if !json_path.exists() {
        fs::File::create(&json_path).unwrap();
    }
    let json = fs::read_to_string(&json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    json
}

pub fn write_an_progress(anime: (&str, &str, &str), progress: &u64) {
    let config_path = dirs::config_dir().unwrap().join("kami");
    let json_path = config_path.join("an_progress.json");
    let json = fs::read_to_string(&json_path).unwrap();
    let mut json: serde_json::Value =
        serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    let mut title_json = serde_json::Map::new();
    title_json.insert(
        "progress".to_string(),
        serde_json::Value::from(progress.clone()),
    );
    title_json.insert("link".to_string(), serde_json::Value::from(anime.1));
    title_json.insert("image".to_string(), serde_json::Value::from(anime.2));
    title_json.insert(
        "updated".to_string(),
        serde_json::Value::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ),
    );
    //insert title_json into json
    if json[anime.0].is_null() {
        json[anime.0] = serde_json::Value::from(title_json);
    } else {
        json[anime.0]["progress"] = serde_json::Value::from(progress.clone());
        json[anime.0]["link"] = serde_json::Value::from(anime.1);
        json[anime.0]["image"] = serde_json::Value::from(anime.2);
        json[anime.0]["updated"] = serde_json::Value::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
    }
    let json = serde_json::to_string_pretty(&json).unwrap();
    fs::write(&json_path, json).unwrap();
}

pub fn get_an_history() -> (Vec<String>, Vec<String>, Vec<String>) {
    //get the titles, links, and images from the json
    let json = get_an_json();
    let mut titles = vec![];
    let mut links = vec![];
    let mut images = vec![];
    let mut last_updated = vec![];
    //if the json is empty, return empty vectors
    if json.is_null() {
        return (titles, links, images);
    }
    for (key, value) in json.as_object().unwrap() {
        titles.push(key.to_string());
        links.push(value["link"].as_str().unwrap().to_string());
        images.push(value["image"].as_str().unwrap().to_string());
        last_updated.push(value["updated"].as_u64().unwrap());
    }
    let mut indices: Vec<usize> = (0..last_updated.len()).collect();
    indices.sort_by(|&a, &b| last_updated[b].cmp(&last_updated[a]));
    titles = indices.iter().map(|&i| titles[i].clone()).collect();
    links = indices.iter().map(|&i| links[i].clone()).collect();
    images = indices.iter().map(|&i| images[i].clone()).collect();
    (links, titles, images)
}

pub fn get_an_progress(title: &str) -> i32 {
    let json = get_an_json();
    let selected = json[title]["progress"].as_u64().unwrap_or(0);
    selected as i32
}
