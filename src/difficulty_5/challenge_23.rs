use school_challenges_rust::utils::*;

fn main() {
    println!("Challenge 23");

    let amount: f64 = input("Enter the amount of money to convert: ")
        .parse::<f64>()
        .expect("Invalid amount");

    let coin_choice: u8 = input(
        "Select a coin to convert into (1: £1, 2: 50p, 3: 20p, 4: 10p, 5: 5p, 6: 2p, 7: 1p): ",
    )
    .parse::<u8>()
    .expect("Invalid choice");

    let coin_value: f64 = match coin_choice {
        1 => 1.0,
        2 => 0.5,
        3 => 0.2,
        4 => 0.1,
        5 => 0.05,
        6 => 0.02,
        7 => 0.01,
        _ => {
            println!("Invalid coin choice.");
            return;
        }
    };

    let num_coins: u32 = (amount / coin_value).floor() as u32;
    println!(
        "You will receive {} coins of value £{:.2}.",
        num_coins, coin_value
    );
}
