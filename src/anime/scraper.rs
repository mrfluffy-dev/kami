use base64::{decode, encode};
use isahc::{ReadResponseExt, Request, RequestExt};
use regex::Regex;

pub fn get_anime_html(url: &str) -> String {
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

pub fn get_ep_location(url: &str) -> String {
    let request = Request::builder()
        .method("HEAD")
        .uri(url)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/100.0",
        )
        .body(())
        .unwrap();
    let response = request.send().unwrap();
    let headers = response.headers();
    let location = headers.get("location").unwrap();
    location.to_str().unwrap().to_string()
}

pub fn anime_names(query: String) -> Vec<String> {
    let url = format!("https://www1.gogoanime.ee//search.html?keyword={}", query);
    //relpace all spaces with %20
    let url = url.replace(' ', "%20");
    let html = get_anime_html(&url);
    let re = Regex::new(r#"(?m)/category/([^"]*)"#).unwrap();
    let mut anime_list: Vec<String> = Vec::new();
    for cap in re.captures_iter(&html) {
        anime_list.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    anime_list.dedup();

    anime_list
}

pub fn anime_ep_range(anime_name: &str) -> u16 {
    let url = format!("https://www1.gogoanime.ee/category/{}", anime_name);
    let re = Regex::new(r#"(?m)\s<a href="\#" class="active" ep_start = (.*?)</a>"#).unwrap();
    let episodes = re
        .captures_iter(&get_anime_html(&url))
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .to_string();
    episodes
        .split('-')
        .nth(1)
        .unwrap_or("0")
        .parse::<u16>()
        .unwrap_or(0)
}

pub fn anime_link(title: &str, ep: u64) -> (String, String) {
    let url = format!("https://animixplay.to/v1/{}", title);
    let html = get_anime_html(&url);
    let re = Regex::new(r#"(?m)\?id=([^&]+)"#).unwrap();
    let id1 = re
        .captures_iter(&html)
        .nth(ep as usize - 1)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .to_string();
    let title = format!("{} Episode {}", title.replace('-', " "), ep);
    let encoded_id1 = encode(&id1);
    let anime_id = encode(format!("{}LTXs3GrU8we9O{}", id1, encoded_id1));
    let html = format!("https://animixplay.to/api/live{}", anime_id);
    let url = get_ep_location(&html);
    let url = url.split('#').nth(1).unwrap();
    let url = std::str::from_utf8(&decode(url).unwrap())
        .unwrap()
        .to_string();
    (url, title)
}
