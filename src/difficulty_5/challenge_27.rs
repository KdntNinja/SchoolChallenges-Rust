use school_challenges_rust::utils::*;

fn main() {
    let distance: u32 = input("Enter the distance (km): ")
        .parse::<u32>()
        .expect("Enter a valid number.");
    let passengers_num: u32 = input("Enter the number of passengers: ")
        .parse::<u32>()
        .expect("Enter a valid number.");
    let mut cost: f32 = (3 + (distance - 1) * 2) as f32;

    if passengers_num >= 5 {
        cost *= 1.5;
    }

    println!("Cost: £{:.2}", cost);
}
