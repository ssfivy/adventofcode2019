use std::fs;
use std::env;

use aoc;

use permutohedron::Heap;

fn main() {
    // Generate permutations
    let mut phases = [0, 1, 2, 3, 4];
    let phase_settings = Heap::new(&mut phases);

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


    let mut maximum = 0;
    for setting in phase_settings {
        let mut x = 0;
        let mut y = 0;
        let mut amp_a = input_orig.clone();
        let mut amp_b = input_orig.clone();
        let mut amp_c = input_orig.clone();
        let mut amp_d = input_orig.clone();
        let mut amp_e = input_orig.clone();
        aoc::runprog(&mut amp_a, &[setting[0], 0], &mut x);
        aoc::runprog(&mut amp_b, &[setting[1], x] , &mut y);
        aoc::runprog(&mut amp_c, &[setting[2], y] , &mut x);
        aoc::runprog(&mut amp_d, &[setting[3], x] , &mut y);
        aoc::runprog(&mut amp_e, &[setting[4], y] , &mut x);
        //println!("{:?}", setting);
        if x > maximum {
            maximum = x;
        }
    }
    println!("{:?}", maximum);
}
