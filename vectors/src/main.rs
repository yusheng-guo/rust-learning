fn main() {
    println!("Hello, world!");

    let _vec1: Vec<i32> = Vec::new();
    let mut vec2  = vec![1, 2, 3, 4, 5];
    vec2.push(6);
    println!("1st: {}", vec2[0]);
    let _second = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("2nd: {}", second),
        None => println!("Mo 2nd value"),
    }
    for i in &mut vec2{
        *i *= 2;
    }
    for i in &vec2{
        println!("{}", i);
    }
    println!("vec length: {}", vec2.len());
    println!("pop: {:?}", vec2.pop());
    println!("vec length: {}", vec2.len());
}
