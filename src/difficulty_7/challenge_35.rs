use school_challenges_rust::utils::*;

fn main() {
    let cars: u16 = input("Enter the number of cars: ")
        .trim()
        .parse::<u16>()
        .unwrap();
    let people: u16 = input("Enter the number of people: ")
        .trim()
        .parse::<u16>()
        .unwrap();

    let seats: u16 = cars * 5;
    if seats >= people {
        println!("\nWe have enough seats.");
    } else {
        let extra_people = people - seats;
        let extra_cars = (extra_people as f32 / 5.0).ceil() as u16;
        println!("\n{} extra cars are needed.", extra_cars);
    }
}
