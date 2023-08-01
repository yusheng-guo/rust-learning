use std::ops::Add;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("nums: {:?}", nums);
    let ret = sum(nums);
    println!("ret: {}", ret);
    let (v1, v2) = add(7);
    println!("v1: {}, v2: {}", v1, v2);

    println!("5 + 4 = {}", sum_gen(5, 4));
}

fn sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for &val in nums.iter(){
        sum += &val;
    }
    sum
}

fn add(num: i32) -> (i32, i32){
    return (&num+1, &num+2)
}

// 泛型
fn sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}