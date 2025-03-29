use school_challenges_rust::utils::*;

fn main() {
    let num1: f64 = input("Enter the first number: ")
        .parse::<f64>()
        .expect("Please enter a valid number");
    let num2: f64 = input("Enter the second number: ")
        .parse::<f64>()
        .expect("Please enter a valid number");

    let sum: f64 = num1 + num2;
    let product: f64 = num1 * num2;

    println!("The sum of the two numbers is: {}", sum);
    println!("The product of the two numbers is: {}", product);
}
