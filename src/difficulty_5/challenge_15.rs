use school_challenges_rust::utils::*;

fn main() {
    let olympic_values: [&str; 3] = ["Respect", "Excellence", "Friendship"];

    let user_input: String =
        input("Name one of the Olympic Values (Respect, Excellence, Friendship): ");

    if olympic_values.contains(&user_input.as_str()) {
        println!("That’s correct");
    } else {
        println!("Incorrect");
    }
}
