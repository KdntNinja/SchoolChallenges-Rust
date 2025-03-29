use school_challenges_rust::utils::*;

fn main() {
    let number: usize = random(1, 11);
    let guess: usize = input("Guess a number between 1 and 10: ")
        .parse::<usize>()
        .expect("Not an integer");

    if guess == number {
        println!("Correct!");
    } else {
        println!("Not what I was thinking");
    }
}
