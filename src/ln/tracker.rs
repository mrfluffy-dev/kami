use serde_json;
use std::fs;
//
//
pub fn get_ln_json() -> serde_json::Value {
    let config_path = dirs::config_dir().unwrap().join("kami");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }
    let json_path = config_path.join("ln_progress.json");
    if !json_path.exists() {
        fs::File::create(&json_path).unwrap();
    }
    let json = fs::read_to_string(&json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    json
}

pub fn write_ln_progress(title: &str, current_page: &u32, selected: &usize) {
    let config_path = dirs::config_dir().unwrap().join("kami");
    let json_path = config_path.join("ln_progress.json");
    let json = fs::read_to_string(&json_path).unwrap();
    let mut json: serde_json::Value =
        serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    let mut title_json = serde_json::Map::new();
    title_json.insert(
        "current_page".to_string(),
        serde_json::Value::from(current_page.clone()),
    );
    title_json.insert(
        "selected".to_string(),
        serde_json::Value::from(selected.clone()),
    );
    //insert title_json into json
    if json[title].is_null() {
        json[title] = serde_json::Value::from(title_json);
    } else {
        json[title]["current_page"] = serde_json::Value::from(current_page.clone());
        json[title]["selected"] = serde_json::Value::from(selected.clone());
    }
    let json = serde_json::to_string_pretty(&json).unwrap();
    fs::write(&json_path, json).unwrap();
}

pub fn get_ln_progress(title: &str) -> (u32, usize) {
    let json = get_ln_json();
    let current_page = json[title]["current_page"].as_u64().unwrap_or(1) as u32;
    let selected = json[title]["selected"].as_u64().unwrap_or(0) as usize;
    (current_page, selected)
}
