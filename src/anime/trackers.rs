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
    } else {
        //read token from file
        let token = fs::read_to_string(&token_path).unwrap();
        if token.is_empty() {
            println!("please go to the below link and copy and past the token below");
            println!(
                "https://anilist.co/api/v2/oauth/authorize?client_id=9121&response_type=token"
            );
            let token = string_input("token: ");
            fs::write(&token_path, token).unwrap();
        }
    }
    let token = fs::read_to_string(&token_path).unwrap();
    token
}

pub fn get_anime_id(anime: &str) -> i32 {
    const QUERY: &str = "
query ($id: Int, $search: String) {
    Media (id: $id, search: $search, type: ANIME) {
        id
        title {
            native
            romaji
            english
        }
    }
}
";
    let json = json!({
        "query": QUERY,
        "variables": {
            "search": anime,
        }
    });
    let resp = Request::builder()
        .method("POST")
        .uri("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .unwrap()
        .send()
        .unwrap()
        .text();
    //println!("{}", resp);
    let regex = regex::Regex::new(r#"id":(.*?),"#).unwrap();
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
    //println!("{}", resp);
    let regex = regex::Regex::new(r#"progress":(.*?)}"#).unwrap();
    let resp: String = resp.as_ref().unwrap().to_string();
    //if resp contains "404"set progress to 1
    // else set progress to the number in the regex
    if resp.contains("404") {
        1
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

pub fn update_anime_progress(anime_id: i32, anime: &str, progress: usize, token: &str) {
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
    println!("updated progress of {} to episode {}", anime, progress);
}
