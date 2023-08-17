use std::fs::File;

fn main() {
    // println!("Hello, world!");
    let path = "lines.txt";
    let file_result = File::create(path);
    let mut output;
    match file_result {
        Ok(file) => output = file,
        Err(err) => {
            panic!("creat file, err: {}", err);
        },
    };
    write!(output, "some words").expect("write to file failed");
}
