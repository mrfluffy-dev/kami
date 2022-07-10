use std::io::{self, Write};

pub fn string_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    input.trim().to_string()
}

pub fn int_input(prompt: &str) -> usize {
    print!("{}", prompt);
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    input.trim().parse::<usize>().unwrap()
}

pub fn u16_input(prompt: &str) -> u16 {
    print!("{}", prompt);
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    input.trim().parse::<u16>().unwrap()
}
