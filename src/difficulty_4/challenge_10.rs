use school_challenges_rust::utils::*;

fn main() {
    let num_of_letters: i8 = 26;
    let guess: i8 = input("How many letters are there in the alphabet? ")
        .parse::<i8>()
        .expect("Not an integer");

    if guess == num_of_letters {
        println!("Correct");
    } else {
        println!("Wrong");
    }
}
