// 原生类型

fn main() {
    // println!("Hello, world!");
    // 1.标量类型 scalar type
    // 整形 浮点型 字符 布尔 单元类型

    // 2. 复合类型 数组 元组

    // 类型说明(普通. 后缀)
    let _f1 = 1f32;
    let _f2: f64 = 2.0;
    // 字面量和运算符

    let tuple = (1, 2.0, "3", true, (1, 2, 3));
    println!("tuple: {:?}", tuple);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple); // 太长的元组无法打印

    // 数组
    // 定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("{}", xs.len());
    println!("{}", ys.len());
}
