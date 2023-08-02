use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let mut heroes = HashMap::new();
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Spider-Man", "Peter Parker");
    heroes.insert("Iron Man", "Tony Stark");
    heroes.insert("The Flash", "Barry Allen");
    heroes.insert("Thor", "Thor Odinson");
    for (k, v) in heroes.iter(){
        println!("{} => {}", k, v);
    }

    println!("The number of heroes: {}", heroes.len());
    println!("The capacity of heroes: {}", heroes.capacity());
    if heroes.contains_key("Batman") {
        let batman = heroes.get("Batman");
        match batman {
            Some(_) => println!("Batman is a hero."),
            None => println!("Batman is not a hero.")
        }
    }

    heroes.clear();
    for (k, v) in heroes.iter(){
        println!("{} => {}", k, v);
    }
}
