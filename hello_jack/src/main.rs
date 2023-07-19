#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you.";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello, {}! {}", name.trim_end(), greeting);
}
