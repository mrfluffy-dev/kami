pub fn open_video(link: (String, String)) {
    let title = link.1;
    let title = title.replace("-", " ");
    let arg: String = format!("--force-media-title={}", title);
    let _ = std::process::Command::new("mpv")
        .arg(link.0)
        .arg(arg)
        .output()
        .expect("failed to open mpv");

    // clear terminal
    print!("\x1b[2J\x1b[1;1H");
}
