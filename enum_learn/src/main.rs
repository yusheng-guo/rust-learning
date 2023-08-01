fn main() {
    println!("Hello, world!");

    #[allow(dead_code)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }
    let today = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Today is weekend: {}", today.is_weekend());
}
