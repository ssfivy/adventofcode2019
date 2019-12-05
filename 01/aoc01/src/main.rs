use std::fs;
use std::io;
use std::io::prelude::*;
use std::env;

fn mass2fuel(mass: i32) -> i32 {
    // Perform integer division, which will round down automagically
    (mass / 3) - 2
}

fn mass2fuel_rec(mass: i32) -> i32 {
    // Perform integer division, which will round down automagically
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + mass2fuel_rec(fuel)
    }
}

fn main() {
    println!("Validating algorithm");
    assert!(mass2fuel(1969) == 654);
    assert!(mass2fuel(100756) == 33583);
    assert!(mass2fuel_rec(1969) == 966);
    assert!(mass2fuel_rec(100756) == 50346);

    // Read file location from stdin
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Opening file {}", filename);
    let input= fs::File::open(filename)
        .expect("Cannot open file");
    let buffered = io::BufReader::new(input);

    let mut fuel1: i32 = 0;
    let mut fuel2: i32 = 0;
    for line in buffered.lines() {
        let module = line.unwrap().parse::<i32>().unwrap();
        fuel1 += mass2fuel(module);
        fuel2 += mass2fuel_rec(module);
    }
    println!("Part 1: {}", fuel1);
    println!("Part 2: {}", fuel2);
}

