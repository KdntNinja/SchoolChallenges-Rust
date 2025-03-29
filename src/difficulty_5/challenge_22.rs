use school_challenges_rust::utils::*;

fn main() {
    let marks: u8 = input("Enter your marks: ")
        .parse::<u8>()
        .expect("Invalid marks");

    match marks {
        10..=19 => println!("Grade 1"),
        20..=29 => println!("Grade 2"),
        30..=39 => println!("Grade 3"),
        40..=49 => println!("Grade 4"),
        50..=59 => println!("Grade 5"),
        60..=69 => println!("Grade 6"),
        70..=79 => println!("Grade 7"),
        80..=89 => println!("Grade 8"),
        90..=100 => println!("Grade 9"),
        _ => println!("Grade U"),
    }
}
