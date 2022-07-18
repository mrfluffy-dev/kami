use std::fs::File;
use std::io::Write;
use crate::{search_ln,chapter_selector,get_full_text,open_bat};
pub fn ln_read(search: &str, chapter: u32){
    //convert search in to Option<&str>
    let ln_url = search_ln(&search);
    let chapter = chapter as f64;
    let mut selected_page = 1;
    if chapter != 0.0{
        selected_page = (chapter/48.0).ceil() as u32;
    }
    loop {
        //make empty tuple called chapter_url with (String, u32, u32)
        let chapter_url = chapter_selector(&ln_url, selected_page);
        selected_page = chapter_url.1;
        let full_text = get_full_text(&chapter_url.0);
        if cfg!(target_os = "windows"){
            use dirs::home_dir;
            let mut home = format!("{:?}",home_dir()).replace("\\\\","/");
            home.drain(0..6);
            home.drain(home.len()-2..home.len());
            let mut file = File::create(format!("{}/AppData/Roaming/log_e",home)).expect("Unable to create file");
            file.write_all(full_text.as_bytes())
                .expect("Unable to write to file");
            file.sync_all().expect("Unable to sync file");
        }else{
            let mut file = File::create("/tmp/log_e").expect("Unable to create file");
            file.write_all(full_text.as_bytes())
                .expect("Unable to write to file");
            file.sync_all().expect("Unable to sync file");
        };
        //open temp.txt in cat for user to read
        let _com = open_bat();
        print!("\x1B[2J\x1B[1;1H");
    }
}
