fn main() {
    // println!("Hello, world!");
    // array
    let arr = [1,2,3,4,5,6,7,8,9];
    println!("arr[0] => {}", arr[0]);
    println!("len(arr) => {}", arr.len());

    // loop
    let mut idx = 0;
    // loop {
    //     if idx >= 9 {
    //         break;
    //     }
    //     if arr[idx] % 2 == 0 {
    //         idx += 1;
    //         continue;
    //     }
    //     println!("val: {}", arr[idx]);
    //     idx += 1;
    // }

    // while
    // while idx < arr.len() {
    //     println!("val: {}", arr[idx]);
    //     idx += 1;
    // }

    // for
    for val in arr.iter(){
        println!("val: {}", val);
    }
}
