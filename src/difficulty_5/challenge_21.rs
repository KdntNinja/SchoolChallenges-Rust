use school_challenges_rust::utils::*;

fn main() {
    let age: u8 = input("Enter your age: ")
        .parse::<u8>()
        .expect("Invalid age");

    match age {
        13..=15 => println!("30% discount"),
        16..=17 => println!("20% discount"),
        50..=u8::MAX => println!("40% discount"),
        _ => println!("No discount"),
    }
}
