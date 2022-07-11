use std::fs::File;
use std::io::Write;
use crate::{search_ln,chapter_selector,get_full_text,open_bat};
pub fn ln_read(){
    let ln_url = search_ln();
    let mut selected_page = 1;
    loop {
        //make empty tuple called chapter_url with (String, u32, u32)
        let chapter_url = chapter_selector(&ln_url, selected_page);
        selected_page = chapter_url.1;
        let full_text = get_full_text(&chapter_url.0);
        //write full_text to file called temp.txt
        let mut file = File::create("/tmp/log_e").expect("Unable to create file");
        file.write_all(full_text.as_bytes())
            .expect("Unable to write to file");
        //close file
        file.sync_all().expect("Unable to sync file");
        //open temp.txt in cat for user to read
        let _com = open_bat();
        print!("\x1B[2J\x1B[1;1H");
    }

}
