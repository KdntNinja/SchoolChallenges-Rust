use school_challenges_rust::utils::*;

fn main() {
    let num_of_letters: u8 = 26;
    let guess: u8 = input("How many letters are there in the alphabet? ")
        .parse::<u8>()
        .expect("Not an integer");

    if guess == num_of_letters {
        println!("Correct");
    } else {
        println!("Wrong");
    }
}
