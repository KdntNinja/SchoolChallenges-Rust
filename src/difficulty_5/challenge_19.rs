use school_challenges_rust::utils::*;

fn main() {
    let num: u8 = 7;

    loop {
        let guess: u8 = input("Enter the number 7: ")
            .parse::<u8>()
            .expect("Invalid input");
        if guess == num {
            println!("Well Done");
            break;
        }
    }
}
