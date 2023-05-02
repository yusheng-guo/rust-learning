use std::{io, cmp::Ordering};
use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 生成随机数

    // println!("The secret number is: {secret_number}");

    loop { 
        println!("Please input your guess 1~100.");

        let mut guess = String::new();
        
        io::stdin() // 获取用户输入
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        // 将字符串转换为数字类型
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        // .expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal=> { // 猜对了 退出游戏
                println!("You win!");
                break;
            },
        }
    }
}
