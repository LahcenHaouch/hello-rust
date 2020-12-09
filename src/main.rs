use ferris_says::say;

use std::io::{stdout, stdin, BufWriter};

fn main() {
    println!("What's your name?");
    let mut name = String::new();
    let stdin = stdin();
    stdin.read_line(&mut name).unwrap();
    let greetings = format!("Hello {}", name);
    let width = greetings.chars().count();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    say(greetings.as_bytes(), width, &mut writer).unwrap();
}
