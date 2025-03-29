use school_challenges_rust::utils::*;

fn main() {
    let name: String = input("Enter your name: ");
    let age: String = input("Enter your age: ");
    let favourite_colour: String = input("Enter your favourite colour: ");

    println!("Hello, {}! You are {} years old and your favourite colour is {}.", name, age, favourite_colour);
}
