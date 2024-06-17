use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let path = &args[2];

    println!("query is: {}", query);
    println!("path is: {}", path);

    let contents = fs::read_to_string(path)
        .expect("not able to read");

    println!("content is: \n{contents}")
}
