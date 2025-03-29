use school_challenges_rust::utils::*;

fn main() {
    let mut drinks = vec![true; 20];
    loop {
        let choice: String = input("Please select a drink (1-20): ");

        if choice == "CANCEL" {
            println!("Selection canceled. Please select again.");
            continue;
        }

        match choice.parse::<usize>() {
            Ok(selection) if selection >= 1 && selection <= 20 => {
                if drinks[selection - 1] {
                    println!("Dispensing your drink...");
                    drinks[selection - 1] = false; // Mark the drink as dispensed (unavailable)
                    break; // End the process after dispensing
                } else {
                    println!("Sorry, this drink is unavailable.");
                }
            }
            _ => {
                println!("Invalid selection. Try again.");
            }
        }
    }
}
