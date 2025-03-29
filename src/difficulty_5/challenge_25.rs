use std::collections::HashMap;

struct RaceFile {
    age_category: String,
}

fn count_age_categories(runners: Vec<RaceFile>) -> HashMap<String, usize> {
    let mut age_category_counts: HashMap<String, usize> = HashMap::new();

    for runner in runners {
        *age_category_counts.entry(runner.age_category).or_insert(0) += 1;
    }

    age_category_counts
}

fn main() {
    let runners: Vec<RaceFile> = vec![
        RaceFile {
            age_category: String::from("U18"),
        },
        RaceFile {
            age_category: String::from("Snr"),
        },
        RaceFile {
            age_category: String::from("Vet"),
        },
        RaceFile {
            age_category: String::from("Snr"),
        },
    ];

    let age_category_counts: HashMap<String, usize> = count_age_categories(runners);

    for (age_category, count) in age_category_counts {
        println!("Age category: {}, Count: {}", age_category, count);
    }
}
