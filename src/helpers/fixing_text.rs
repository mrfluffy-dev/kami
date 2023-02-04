use regex::Regex;

//function that takes a vector called ln_chapters of strings and removes everyting after the first occurence of "-" and all \ and "
pub fn remove_after_dash(ln_chapters: &Vec<String>) -> Vec<String> {
    let mut ln_chapters_new: Vec<String> = Vec::new();
    let re = Regex::new(r#"\\"(.*?) -"#).unwrap();
    for ln in ln_chapters {
        for cap in re.captures_iter(ln) {
            ln_chapters_new.push(cap.get(1).unwrap().as_str().trim().to_string());
        }
    }
    ln_chapters_new = replace_unicode(&ln_chapters_new);
    ln_chapters_new
}

//function that takes a vector called ln_chapters and looks for unicode characters and replaces them with the ascii version
pub fn replace_unicode(ln_chapters: &Vec<String>) -> Vec<String> {
    let mut ln_chapters_new: Vec<String> = Vec::new();
    for ln in ln_chapters {
        //make regex to find all \uxxxx and save it in to a vector
        let re = Regex::new(r#"(\\u[0-9a-fA-F]{4})"#).unwrap();
        let mut vec_unicode: Vec<String> = Vec::new();
        for cap in re.captures_iter(ln) {
            vec_unicode.push(cap.get(1).unwrap().as_str().to_string());
        }
        let mut ln_new: String = String::new();
        if !vec_unicode.is_empty() {
            //loop through the vector and replace the unicode characters with the ascii version
            for unicode in vec_unicode {
                //convert the unicode to char
                let unicode_char =
                    char::from_u32(u32::from_str_radix(&unicode[2..6], 16).unwrap()).unwrap();
                let unicode_str = unicode_char as char;
                ln_new = ln.replace(&unicode, &unicode_str.to_string());
            }
        } else {
            ln_new = ln.to_string();
        }
        ln_chapters_new.push(ln_new);
    }
    ln_chapters_new
}

pub fn fix_html_encoding(ln_text: &Vec<String>) -> Vec<String> {
    let mut ln_text_new: Vec<String> = Vec::new();
    for ln in ln_text {
        let ln = ln.replace("&#8213;", "--");
        let ln = ln.replace("&#8214;", "--");
        let ln = ln.replace("&#8216;", "'");
        let ln = ln.replace("&#8217;", "'");
        let ln = ln.replace("&#8220;", "\"");
        let ln = ln.replace("&#8221;", "\"");
        let ln = ln.replace("&#8230;", "...");
        let ln = ln.replace("&#8242;", "'");
        let ln = ln.replace("&#8243;", "\"");
        let ln = ln.replace("&#8260;", "--");
        let ln = ln.replace("&#8212;", "--");
        ln_text_new.push(ln);
    }
    ln_text_new
}

pub fn htmlise(query: String) -> String {
    query
        .replace(" ", "%20")
        .replace(":", "%3A")
        .replace("!", "%21")
        .replace("?", "%3F")
        .replace("'", "%27")
}
