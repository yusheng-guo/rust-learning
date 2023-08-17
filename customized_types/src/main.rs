// 自定义类型
// 1.结构体 struct [structure]
// 1.1 元组结构体
// struct Pair(f32, f32);


// 1.2 C语言风格结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 1.3 单元结构体
struct Unit;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// TODO: fix
impl Rectangle{
    fn rect_area(self: Self) -> f32{
        return (self.bottom_right.x-self.top_left.x) * (self.bottom_right.y-self.top_left.y)
    }

    // fn square(self: &Self, p: *Point, mut f: f32 ) -> Rectangle{
    //     return Rectangle{
    //         top_left: *p,
    //         bottom_right: Point{x: p.x+ *f, y: p.y+*f},
    //     }
    // }
}


// 2.枚举
enum WebEvent {
    PageLoad, // 单元结构体（称为 `unit-like` 或 `unit`）
    PageUnload,
    KeyPress(char), // 元组结构体
    Paste(String),
    Click{x: f32, y: f32}, // 普通结构体
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}



fn main() {
    // println!("Hello, world!");
    let peter: Person = Person{name:"peter".to_string(), age:20};
    println!("name: {} ,age: {}", peter.name, peter.age);
    println!("{:?}", peter);

    let mut rect: Rectangle = Rectangle{
        top_left: Point{x:10.0, y:10.0},
        bottom_right: Point{x:20.0, y:100.0},
    };
    println!("{:?}", rect);

    println!("{:?}", rect.rect_area());
    // rect = rect.square(Point{x: 1., y: 2.}, &10.);
    // println!("{:?}", rect);
    // 实例化一个单元结构体
    let _unit = Unit;


    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20., y: 80. };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);


    // 3.常量 [constant] const static 关键字
}
