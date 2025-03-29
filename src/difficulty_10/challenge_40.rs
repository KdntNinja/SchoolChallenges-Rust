use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut lives = 3;
    let mut points = 0;
    let mut level = 1;

    println!("Welcome to the game!");
    println!(
        "You have {} lives and need to reach 20 points to finish the game.",
        lives
    );

    while lives > 0 && points < 20 {
        println!("\nLevel {} - Points: {} - Lives: {}", level, points, lives);

        // Simulate the user making a move (gameplay)
        let success: bool = rand::rng().random_range(0..2) == 0; // 50% chance of success or failure

        if success {
            points += 1;
            println!("You gained 1 point!");
        } else {
            lives -= 1;
            println!("You lost a life!");
        }

        // Check if the player has reached the next level
        if points >= level * 5 {
            level += 1;
            println!("Congratulations! You've moved to Level {}.", level);
        }

        // Wait for user input before continuing to the next round
        print!("\nPress Enter to continue...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    // Game over condition
    if points >= 20 {
        println!(
            "\nYou completed the game! You reached Level {} with {} points.",
            level - 1,
            points
        );
    } else {
        println!(
            "\nGame over! You reached Level {} with {} points.",
            level - 1,
            points
        );
    }
}
