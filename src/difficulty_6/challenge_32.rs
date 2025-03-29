use school_challenges_rust::utils::*;

fn main() {
    // Get the initial savings amount
    let saving_amount: f64 = input("Enter the amount to save (£): ")
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Get the number of bank accounts to compare
    let num_accounts: usize = input("Enter the number of bank accounts to compare: ")
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Iterate over each bank account
    for i in 1..=num_accounts {
        let interest_rate: f64 = input(&format!("Enter the interest rate for account {}: ", i))
            .trim()
            .parse()
            .expect("Please enter a valid number");

        // Calculate interest and total amount
        let interest = (saving_amount / 100.0) * interest_rate;
        let total = saving_amount + interest;

        println!(
            "Bank Account {}: Interest = {:.2}, Total = {:.2}\n",
            i, interest, total
        );
    }
}
