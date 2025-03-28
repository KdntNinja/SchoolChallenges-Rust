use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut fuel_prices: HashMap<i8, f32> = HashMap::new();
    fuel_prices.insert(1, 1.40); // Petrol
    fuel_prices.insert(2, 1.55); // Diesel
    fuel_prices.insert(3, 0.95); // LPG

    let mut fuel_type: String = Default::default(); // Converted into i8 later
    print!("(1) Petrol\n(2) Diesel\n(3) LPG\n: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fuel_type).expect("Failed to read input");
    let fuel_type: i8 = fuel_type.trim().parse::<i8>().unwrap_or(-1); // Default to -1 if input is invalid
    
    
    let mut fuel_amount: String = Default::default(); // Converted into f32 later
    print!("Enter the amount of fuel (L): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fuel_amount).expect("Failed to read input");
    let fuel_amount: f32 = fuel_amount.trim().parse::<f32>().unwrap();

    match fuel_prices.get(&fuel_type) {
        Some(price) => println!("Price of fuel: £{:.2}", price*fuel_amount),
        None => println!("Invalid choice. Please enter 1, 2, or 3."),
    }
    
    let mut payed_amount: String = Default::default(); // Converted into f32 later
    print!("Enter the amount of payed (£): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut payed_amount).expect("Failed to read input");
    let payed_amount: f32 = payed_amount.trim().parse::<f32>().unwrap();
    
    println!("You have £{} to pay.", payed_amount-fuel_amount);
}