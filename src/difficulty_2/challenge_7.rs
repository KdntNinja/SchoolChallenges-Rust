use school_challenges_rust::utils::*;

fn calculate_average_speed(distance: f64, time: f64) -> f64 {
    distance / time
}

fn main() {
    let distance: f64 = input("Enter the distance (in metres): ")
        .parse::<f64>()
        .expect("Please enter a valid number");
    let time: f64 = input("Enter the time (in seconds): ")
        .parse::<f64>()
        .expect("Please enter a valid number");

    let average_speed: f64 = calculate_average_speed(distance, time);

    println!("The average speed is: {} m/s", average_speed);
}
