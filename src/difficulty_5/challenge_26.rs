use school_challenges_rust::utils::*;

fn main() {
    let dog_age: u32 = input("Enter the age of the dog (y): ")
        .parse::<u32>()
        .expect("Please enter a valid number");

    let human_age: u32 = if dog_age <= 2 {
        dog_age * 12
    } else {
        24 + (dog_age - 2) * 6
    };

    println!(
        "The human equivalent of a {} year old dog is {} years old.",
        dog_age, human_age
    );
}
