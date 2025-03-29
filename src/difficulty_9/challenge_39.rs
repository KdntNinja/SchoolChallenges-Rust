use rand::prelude::*;
use rand::rngs::ThreadRng;
use school_challenges_rust::utils::*;
use std::io::{self, Write};

fn generate_question(rng: &mut ThreadRng) -> (usize, usize, String) {
    let num1: usize = rng.random_range(1..=10);
    let num2: usize = rng.random_range(1..=10);
    let operations = vec!["+", "-", "*"];
    let operation = operations.choose(rng).unwrap();

    if *operation == "-" && num1 < num2 {
        return (num2, num1, operation.to_string());
    }

    (num1, num2, operation.to_string())
}

fn ask_questions() -> u32 {
    let mut rng: ThreadRng = rand::rng();
    let mut score: u32 = 0;

    let name: String = input("Please enter your name: ")
        .trim()
        .parse::<String>()
        .unwrap();

    println!("Hello, {}! Let's start the quiz.", name);

    for i in 1..=10 {
        let (num1, num2, operation) = generate_question(&mut rng);
        println!("Question {}: What is {} {} {}?", i, num1, operation, num2);

        let correct_answer = match operation.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            _ => 0,
        };

        print!("Your answer: ");
        io::stdout().flush().unwrap();

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).unwrap();
        let user_answer: usize = user_answer.trim().parse().unwrap_or(0);

        if user_answer == correct_answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect. The correct answer is: {}", correct_answer);
        }
    }

    println!("Quiz over! Your score is: {}/10", score);
    score
}

fn main() {
    ask_questions();
}
