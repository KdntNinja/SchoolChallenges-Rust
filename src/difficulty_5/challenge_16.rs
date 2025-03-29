use school_challenges_rust::utils::*;

fn main() {
    let time: u8 = input("How much TV have you watched (h)? ")
        .trim()
        .parse::<u8>()
        .unwrap();

    match time {
        0..=2 => println!("That should be ok"),
        3..=4 => println!("That will rot your brain"),
        _ => println!("That is too much TV"),
    }
}
