use school_challenges_rust::utils::*;

fn main() {
    let hours_worked: u32 = input("Enter the hours worked: ")
        .parse::<u32>()
        .expect("Expecting a valid integer");
    let bears_made: u32 = input("Enter the number of bears made: ")
        .parse::<u32>()
        .expect("Expecting a valid integer");

    let wages_from_hours: u32 = hours_worked * 5;
    let wages_from_bears: u32 = bears_made * 2;

    let wages: u32 = wages_from_hours.max(wages_from_bears);

    println!("The total wages earned: £{}", wages);
}
