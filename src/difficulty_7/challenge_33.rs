use school_challenges_rust::utils::*;

fn main() {
    let gcse_num: u8 = input("Enter the number of GCSEs you have: ")
        .parse::<u8>()
        .expect("Please enter a valid number");

    let mut total_points = 0;

    for i in 1..=gcse_num {
        let grade: u8 = input(&format!("Enter the grade for GCSE {}: ", i))
            .parse::<u8>()
            .expect("Please enter a valid grade (0-9)");
        total_points += grade;
    }

    println!("Total points: {}", total_points);

    if total_points >= 40 {
        println!("You can go to the sixth form.");
    } else if total_points >= 35 {
        println!("A discussion is needed.");
    } else {
        println!("Sorry, not enough points.");
    }
}
