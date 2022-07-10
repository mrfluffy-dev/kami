use std::io::Result;
use std::process::{Command, ExitStatus};

pub fn open_bat() -> Result<ExitStatus> {
    Command::new("bat")
        .arg("--paging")
        .arg("always")
        .arg("/tmp/log_e")
        .spawn()?
        .wait()
}
