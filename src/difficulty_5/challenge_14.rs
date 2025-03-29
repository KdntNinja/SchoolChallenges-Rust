use school_challenges_rust::utils::*;

fn main() {
    let traffic_light: String =
        input("Enter the traffic light colour (green, amber, red): ").to_lowercase();

    if traffic_light == "green" {
        println!("Go.");
    } else if traffic_light == "amber" {
        println!("Get Ready.");
    } else {
        println!("Stop.");
    }
}
