// fn main() {
//     // println!("Hello, world!");
//
//     // 所有权
//     { // s 在这里无效，它尚未声明
//         let s = "hello"; // 从此处起，s 是有效的
//         println!("{}", s) // 使用 s
//     } // 此作用域已结束，s 不再有效
//
//     let mut s = String::from("hello");
//     s.push_str(", world.");
//     println!("{}", s)
// }

// 当参数类型实现了 Copy trait 时
// 传递参数时会进行值的复制 而不是进行所有权的转移
// 这种行为被称为“传值”，而不是“传引用”。
// copy 与 clone 不同
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x
    println!("{}", x);
    println!("{}", s);
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
