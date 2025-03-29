use school_challenges_rust::utils::input;

fn main() {
    let name: String = input("Enter your name: ");
    let age: String = input("Enter your age: ");
    let favourite_colour: String = input("Enter your favourite colour: ");

    println!(
        "\nName: {}\nAge: {}\nFavourite Colour:{}",
        name, age, favourite_colour
    );
}
