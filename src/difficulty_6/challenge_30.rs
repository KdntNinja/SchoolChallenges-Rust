use school_challenges_rust::utils::*;
use std::collections::HashSet;

fn has_duplicate(vec: &[u32]) -> bool {
    let mut seen = HashSet::new();
    for &num in vec {
        if !seen.insert(num) {
            return true; // Duplicate found
        }
    }
    false
}

fn main() {
    let mut sides: Vec<u32> = vec![];

    for i in 1..=3 {
        eprint!("Input length {}: ", i);
        let length: u32 = input("")
            .trim()
            .parse()
            .expect("Please enter a valid number");
        sides.push(length);
    }

    println!("Is isosceles: {}", has_duplicate(&*sides));
}
