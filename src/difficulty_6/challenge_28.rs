use school_challenges_rust::utils::*;
use std::f32::consts::PI;

fn main() {
    let length: u32 = input("Enter the length of the lawn: ")
        .parse::<u32>()
        .expect("Expecting a valid integer");
    let width: u32 = input("Enter the width of the lawn: ")
        .parse::<u32>()
        .expect("Expecting a valid integer");
    let radius: u32 = input("Enter the radius of the circle: ")
        .parse::<u32>()
        .expect("Expecting a valid integer");

    let lawn_area: f32 = (length * width) as f32;
    let flower_bed_area: f32 = PI * (radius.pow(2) as f32);
    let turf_needed: f32 = lawn_area - flower_bed_area;

    println!("Turf needed: {:.2} square meters", turf_needed);
}
