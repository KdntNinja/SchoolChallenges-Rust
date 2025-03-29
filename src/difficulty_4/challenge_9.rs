use school_challenges_rust::utils::*;

fn main() {
    let stored_name: &str = "Kaiden";

    let user_name: String = input("Enter your first name: ");

    if user_name == stored_name {
        println!("You’re cool.");
    } else {
        println!("Nice to meet you.");
    }
}
