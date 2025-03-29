use school_challenges_rust::utils::*;

fn main() {
    let num1: f64 = input("Enter the first number: ").parse::<f64>().expect("Please enter a valid number");
    let num2: f64 = input("Enter the second number: ").parse::<f64>().expect("Please enter a valid number");

    let total: f64 = num1 + num2;

    println!("The total is: {}", total);
}
