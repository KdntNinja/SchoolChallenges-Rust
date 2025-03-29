use school_challenges_rust::utils::*;

fn main() {
    const FULL_TIME_DAYS: f32 = 5.0;
    const FULL_TIME_HOLIDAY: f32 = 28.0;

    let days_per_week: f32 = input("Enter number of days worked per week: ")
        .parse::<f32>()
        .expect("Not a valid number");

    let holiday_allowance = (days_per_week / FULL_TIME_DAYS) * FULL_TIME_HOLIDAY;
    println!(
        "You get {:.1} days of holiday per year.",
        holiday_allowance
    );
}
