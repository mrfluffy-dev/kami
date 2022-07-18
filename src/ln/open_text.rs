use std::io::Result;
use std::process::{Command, ExitStatus, Stdio};

#[allow(unused_assignments)]
pub fn open_bat() -> Result<ExitStatus> {
    let termsize::Size {rows: _, cols} = termsize::get().unwrap();
    let mut path = String::new();
    if cfg!(target_os = "windows"){
        use dirs::home_dir;
        let mut home = format!("{:?}",home_dir()).replace("\\\\","/");
        home.drain(0..6);
        home.drain(home.len()-2..home.len());
        path = format!("{}/AppData/Roaming/log_e",home).to_string();
    }
    else{
        path = "/tmp/log_e".to_string();
    }

    let soft_wrap = match Command::new("fold")
        .arg("-s")
        .arg("-w")
        .arg((cols - 9).to_string())
        .arg(path)
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
