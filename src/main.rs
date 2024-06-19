#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::env;
use std::process;
use mgrip::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = mgrip::run(config){
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }
}
