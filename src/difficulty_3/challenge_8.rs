use school_challenges_rust::utils::*;

fn calculate_total_cost(minutes: u32, texts: u32) -> f64 {
    let minutes_cost: f64 = minutes as f64 * 0.10;
    let texts_cost: f64 = texts as f64 * 0.05;
    let monthly_charge: f64 = 10.00;

    minutes_cost + texts_cost + monthly_charge
}

fn main() {
    let minutes: u32 = input("Enter the number of minutes used: ")
        .parse::<u32>()
        .expect("Please enter a valid number");
    let texts: u32 = input("Enter the number of texts used: ")
        .parse::<u32>()
        .expect("Please enter a valid number");

    let total_cost: f64 = calculate_total_cost(minutes, texts);

    println!("The total cost of the bill is: £{:.2}", total_cost);
}
