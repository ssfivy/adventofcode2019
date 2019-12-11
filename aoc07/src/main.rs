use std::fs;
use std::env;

use aoc;



fn main() {

    //println!("{:?}", in5);

    // Read file location from stdin
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Opening file {}", filename);
    let prog = fs::read_to_string(filename)
        .expect("Cannot open file");

    //println!("{:?}", prog);
    let input_orig: Vec<i32> = prog
        .trim() // remove last \n
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    //println!("{:?}", input);

    let mut input1 = input_orig.clone();
    aoc::runprog(&mut input1);
}
