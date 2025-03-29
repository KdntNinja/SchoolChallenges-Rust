use rand::Rng;
use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn random(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
