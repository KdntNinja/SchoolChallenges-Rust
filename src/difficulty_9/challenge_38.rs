use school_challenges_rust::utils::*;

fn main() {
    let amount_saved: f64 = input("Enter the amount saved each year: ")
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number");

    let years: u32 = input("Enter the number of years: ")
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid number");

    let mut start_value = 0.0;

    for year in 1..=years {
        let paid_in = amount_saved;
        let interest = (start_value + paid_in) * 0.1;
        let final_value = start_value + paid_in + interest;

        println!(
            "Year {}: Start: {:.2} Paid in: {:.2} Interest: {:.2} Final: {:.2}",
            year, start_value, paid_in, interest, final_value
        );

        start_value = final_value; // The final value becomes the start value for the next year
    }
}
