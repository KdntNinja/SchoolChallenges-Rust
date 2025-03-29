use rand::Rng;
use rand::rngs::ThreadRng;
use school_challenges_rust::utils::*;

fn main() {
    let mut rng: ThreadRng = rand::rng();
    let choices: Vec<&str> = vec!["rock", "paper", "scissors"];

    let computer_choice: &str = choices[rng.random_range(0..choices.len())];

    let mut user_choice: String = input("user> ").to_lowercase().replace(" ", "");

    while !choices.contains(&user_choice.as_str()) {
        println!("Invalid choice! Please choose between 'rock', 'paper', or 'scissors'.");
        user_choice = input("user> ").to_lowercase().replace(" ", "");
    }

    println!("Computer chose: {}", computer_choice);

    match (computer_choice, &user_choice as &str) {
        (x, y) if x == y => println!("Game Tied."),
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => {
            println!("Computer Wins.")
        }
        _ => println!("You Win!"),
    }
}
