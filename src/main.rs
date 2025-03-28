use std::collections::HashMap;
use std::io::{self, Write};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let fuel_prices = HashMap::from([(1, 1.40), (2, 1.55), (3, 0.95)]);
    
    let fuel_type: i8 = get_input("(1) Petrol\n(2) Diesel\n(3) LPG\n: ")
        .parse()
        .unwrap_or(-1);
    
    let fuel_amount: f32 = get_input("Enter the amount of fuel (L): ")
        .parse()
        .expect("Invalid fuel amount");
    
    if let Some(price) = fuel_prices.get(&fuel_type) {
        let total_price = price * fuel_amount;
        println!("Price of fuel: £{:.2}", total_price);
        
        let paid_amount: f32 = get_input("Enter the amount paid (£): ")
            .parse()
            .expect("Invalid payment amount");
        
        let change = total_price - paid_amount;
        println!("You have £{:.2} to pay.", change);
        
        let has_loyalty_card = get_input("Do you have a loyalty card? (y/n): ")
            .to_lowercase();
        
        if has_loyalty_card == "y" {
            let mut points = fuel_amount as i32 + total_price.floor() as i32;
            if points > 100 {
                points = (points as f32 * 1.1) as i32;
            }
            println!("You have earned {} loyalty points.", points);
        }
    } else {
        println!("Invalid choice. Please enter 1, 2, or 3.");
    }
}
