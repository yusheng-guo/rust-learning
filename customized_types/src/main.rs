fn main() {
    // println!("Hello, world!");
    // 自定义类型
    // 1.结构体 struct [structure]
    // 1.1 元组结构体
    struct Pair(f32, f32);

    // 1.2 C语言风格结构体
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // 1.3 单元结构体
    struct Unit;

    struct Point {
        x: f32,
        y: f32,
    }

    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    // 2.枚举

    // 3.常量 [constant] const static 关键字
}
