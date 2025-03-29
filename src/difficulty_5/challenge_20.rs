#[derive(Clone)]
struct MyHotPizza {
    large: u8,
    medium: u8,
    small: u8,
    card: bool,
}

impl MyHotPizza {
    fn new(large: u8, medium: u8, small: u8, card: bool) -> Self {
        MyHotPizza {
            large,
            medium,
            small,
            card,
        }
    }

    fn total_pizzas(&self) -> u8 {
        self.large + self.medium + self.small
    }

    fn display(&self) {
        println!(
            "\nLarge: {}\nMedium: {}\nSmall: {}\nTotal: {}\nCard: {}",
            self.large,
            self.medium,
            self.small,
            self.total_pizzas(),
            self.card
        );
    }
}

fn main() {
    let smith: MyHotPizza = MyHotPizza::new(5, 6, 1, false);
    let williams: MyHotPizza = MyHotPizza::new(10, 12, 3, false);

    let people: Vec<MyHotPizza> = vec![smith, williams];

    for i in 0..people.len() {
        let mut person: MyHotPizza = people[i].clone();

        if person.total_pizzas() >= 20 {
            person.card = true;
        }
        person.display();
    }
}
