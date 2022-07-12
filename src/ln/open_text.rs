use std::io::Result;
use std::process::{Command, ExitStatus, Stdio};

pub fn open_bat() -> Result<ExitStatus> {
    let soft_wrap = match Command::new("fold")
        .arg("-s")
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
        .stdin(soft_wrap.stdout.unwrap())
        .spawn()?
        .wait()
}
