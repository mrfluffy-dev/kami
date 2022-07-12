use std::io::Result;
use std::process::{Command, ExitStatus, Stdio};

pub fn open_bat() -> Result<ExitStatus> {
    let terminal_cols_cmd = Command::new("tput").arg("cols").output()?.stdout;
    let terminal_cols: String = match std::str::from_utf8(&terminal_cols_cmd) {
        Err(_e) => "80".to_string(),
        Ok(v) => (v.trim().parse::<i32>().unwrap() - 10).to_string(),
    };

    let soft_wrap = match Command::new("fold")
        .arg("-s")
        .arg("-w")
        .arg(terminal_cols)
        .arg("/tmp/log_e")
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(soft_wrap) => soft_wrap,
    };

    Command::new("bat")
        .arg("--paging")
        .arg("always")
        .arg("-l")
        .arg("markdown")
        .stdin(soft_wrap.stdout.unwrap())
        .spawn()?
        .wait()
}
