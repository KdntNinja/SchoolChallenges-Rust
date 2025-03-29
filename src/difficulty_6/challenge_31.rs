use school_challenges_rust::utils::*;

fn main() {
    let mut weights: Vec<u32> = vec![];
    let num_of_fruits: u32 = input("Enter the num of fruits: ")
        .parse::<u32>()
        .expect("Not a valid integer");

    for i in 1..=num_of_fruits {
        eprint!("Input weight {}: ", i);
        let weight: u32 = input("")
            .trim()
            .parse()
            .expect("Please enter a valid number");
        weights.push(weight);
    }

    let average: f32 = weights.iter().sum::<u32>() as f32 / weights.len() as f32;
    println!("Average weight is: {}", average);
}
