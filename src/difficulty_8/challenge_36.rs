use school_challenges_rust::utils::*;
use std::collections::HashMap;

fn main() {
    let fuel_prices: HashMap<u8, f32> = HashMap::from([(1, 1.40), (2, 1.55), (3, 0.95)]);

    let fuel_type: u8 = input("(1) Petrol\n(2) Diesel\n(3) LPG\n: ")
        .parse::<u8>()
        .unwrap_or(0);

    let fuel_amount: f32 = input("Enter the amount of fuel (L): ")
        .parse::<f32>()
        .expect("Invalid fuel amount");

    if let Some(price) = fuel_prices.get(&fuel_type) {
        let total_price: f32 = price * fuel_amount;
        println!("Price of fuel: £{:.2}", total_price);

        let paid_amount: f32 = input("Enter the amount paid (£): ")
            .parse::<f32>()
            .expect("Invalid payment amount");

        let change: f32 = paid_amount - total_price;
        if change >= 0.0 {
            println!("Your change is £{:.2}.", change);
        } else {
            println!("You have £{:.2} to pay.", -change);
        }

        let has_loyalty_card: String = input("Do you have a loyalty card? (y/n): ").to_lowercase();

        if has_loyalty_card == "y" {
            let mut points: i32 = fuel_amount as i32 + total_price.floor() as i32;
            if points > 100 {
                points = (points as f32 * 1.1) as i32;
            }
            println!("You have earned {} loyalty points.", points);
        }
    } else {
        println!("Invalid choice. Please enter 1, 2, or 3.");
    }
}
