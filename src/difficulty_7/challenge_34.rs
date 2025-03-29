use school_challenges_rust::utils::*;

fn main() {
    let mut electricity: Vec<u16> = vec![];
    let days: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    for i in 0..days.len() {
        print!("Enter the amount used on {}: ", days[i]);
        let usage: u16 = input("").parse::<u16>().unwrap();
        electricity.push(usage);
    }

    let day: usize = electricity
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .unwrap()
        .0;

    println!("\n{}'s electricity is free.", days[day]);
}
