// struct Rectangle {
//     length: f32,
//     height: f32,
// }

// 泛型
// struct Rectangle<T, U> {
//     length: T,
//     height: U,
// }

use std::f32::consts::PI;

trait Shape{
    fn new(params: &[f32]) -> Self;
    fn area(&self) -> f32;
}

struct Rectangle {
    length: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

impl Shape for Rectangle{
    fn new(params: &[f32]) -> Self {
        let l : f32;
        let h : f32;
        let first_value = params.first();
        match first_value {
            Some(&x) => l = x,
            None => l = 1.0,
        }
        let last_value = params.last();
        match last_value {
            Some(&x) => h = x,
            None => h = 1.0,
        }
        return Rectangle{length: l, height: h};
    }
    fn area(&self) -> f32 {
        return &self.length * &self.height;
    }
}

impl Shape for Circle{
    fn new(params: &[f32]) -> Self {
        let r : f32;
        let first_value = params.first();
        match first_value {
            Some(&x) => r = x,
            None => r = 1.0,
        }
        return Circle{radius: r};
    }
    fn area(&self) -> f32 {
        return self.radius.powf(2.0) * PI;
    }
}

fn main() {
    // println!("Hello, world!");
    let rect : Rectangle = Shape::new(&[11.0, 7.0]);
    println!("the area of rect: {}", rect.area());

    let circ : Circle = Shape::new(&[5.0]);
    println!("the area of circ: {}", circ.area());
}
