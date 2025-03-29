use school_challenges_rust::utils::*;

fn main() {
    let num1: i16 = input("How many letters are there in the alphabet? ")
        .parse::<i16>()
        .expect("Not an integer");
    let num2: i16 = input("How many letters are there in the alphabet? ")
        .parse::<i16>()
        .expect("Not an integer");
    if num1 > num2 {
        println!("{}", num1);
    } else if num2 > num1 {
        println!("{}", num2);
    } else {
        println!("The same");
    }
}

// Challenge 11
//
// The program asks the user to input two numbers.
// It will then output the larger of these two numbers.
