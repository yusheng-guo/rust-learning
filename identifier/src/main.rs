#![allow(unused)] // 忽略未使用变量

use rand::Rng;

fn main() {
   // 1.常量
   // const ONE_MIL: u32 = 1_1000_000;
   // const PI: f32 = 3.1415926;
   // let age = "20";
   // let mut age: u32 = age.trim().parse()
   //     .expect("age wasn't assigned a number");
   // age += 1;
   // println!("I'm {} and I want ${}.", age, ONE_MIL);

   // 2.静态类型 - 整形 浮点型
   // u8 u16 u32 u64 u128 usize
   // i8 i16 i32 i64 i128 isize
   // println!("Max u8 : {}", u8::MAX);
   // println!("Max u16 : {}", u16::MAX);
   // println!("Max u32 : {}", u32::MAX);
   // println!("Max u64 : {}", u64::MAX);
   // println!("Max u128 : {}", u128::MAX);
   // println!("Max usize : {}", usize::MAX);
   //
   // println!("Max f32 : {}", f32::MAX);
   // println!("Max f64 : {}", f64::MAX);
   //  布尔类型
    let mut _is_true = true; // _is_true 标识符前加下划线 只作占位 编译器忽略
    let mut _is_false = false;
    println!("_is_true: {}.", _is_true);
    println!("_is_false: {}.", _is_false);

    // 随机值
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random: {}.", random_number);
}
