pub fn open_video(link: (String, String)) {
    let _ = std::process::Command::new("mpv")
        .arg(link.0)
        .output()
        .expect("failed to open mpv");

    // clear terminal
    print!("\x1b[2J\x1b[1;1H");
}
