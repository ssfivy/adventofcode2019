use std::fs;
use std::env;

use aoc;

use permutohedron::Heap;

fn part2_calc (setting: [i32; 5] , input_orig: &Vec<i32>) -> i32 {
        let mut a_pc = 0;
        let mut b_pc = 0;
        let mut c_pc = 0;
        let mut d_pc = 0;
        let mut e_pc = 0;
        let mut a_incnt = 0;
        let mut b_incnt = 0;
        let mut c_incnt = 0;
        let mut d_incnt = 0;
        let mut e_incnt = 0;
        let mut a_out = 0;
        let mut b_out = 0;
        let mut c_out = 0;
        let mut d_out = 0;
        let mut e_out = 0;
        let mut amp_a = input_orig.clone();
        let mut amp_b = input_orig.clone();
        let mut amp_c = input_orig.clone();
        let mut amp_d = input_orig.clone();
        let mut amp_e = input_orig.clone();
        println!("{:?}", setting);
        loop {
            //println!("==========={:?} {:?}", e_out, a_out);
            aoc::runprog(&mut amp_a, &mut a_pc, &mut a_incnt, &[setting[0], e_out] , &mut a_out);
            println!(" {:?}", a_out);
            aoc::runprog(&mut amp_b, &mut b_pc, &mut b_incnt, &[setting[1], a_out] , &mut b_out);
            println!(" {:?}", b_out);
            aoc::runprog(&mut amp_c, &mut c_pc, &mut c_incnt, &[setting[2], b_out] , &mut c_out);
            println!(" {:?}", c_out);
            aoc::runprog(&mut amp_d, &mut d_pc, &mut d_incnt, &[setting[3], c_out] , &mut d_out);
            println!(" {:?}", d_out);
            let end = aoc::runprog(&mut amp_e, &mut e_pc, &mut e_incnt, &[setting[4], d_out] , &mut e_out);
            println!(" {:?}", e_out);

            if end {
                break;
            }
        }
        return e_out;
}

fn main() {

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

    /*
    // Generate permutations
    let mut phases = [0, 1, 2, 3, 4];
    let phase_settings = Heap::new(&mut phases);

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
    println!("Part 1: {:?}", maximum);
    */


    // Part 2
    // Test
    let setting = [9,8,7,6,5];
    let mut input = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    let out = part2_calc(setting, &mut input);
    println!("--->{:?}", out);


    // Generate permutations
    let mut phases = [5,6,7,8,9];
    let phase_settings = Heap::new(&mut phases);

    let mut maximum = 0;
    for setting in phase_settings {
        let out = part2_calc(setting, &input_orig);
        if out > maximum {
            maximum = out;
        }
    }
    println!("Part 2: {:?}", maximum);
}
