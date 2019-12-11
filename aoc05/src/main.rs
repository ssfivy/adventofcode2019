use std::fs;
use std::env;

mod cpu;



fn main() {
    println!("Validating algorithm");
    let mut in1: [i32; 5] = [1,0,0,0,99];
    let     out1: [i32; 5] = [2,0,0,0,99];
    let mut in2: [i32; 5] = [2,3,0,3,99];
    let     out2: [i32; 5] = [2,3,0,6,99];
    let mut in3: [i32; 6] = [2,4,4,5,99,0];
    let     out3: [i32; 6] = [2,4,4,5,99,9801];
    let mut in4: [i32; 9] = [1,1,1,4,99,5,6,0,99];
    let     out4: [i32; 9] = [30,1,1,4,2,5,6,0,99];
    cpu::runprog(&mut in1);
    cpu::runprog(&mut in2);
    cpu::runprog(&mut in3);
    cpu::runprog(&mut in4);
    assert!(in1 == out1);
    assert!(in2 == out2);
    assert!(in3 == out3);
    assert!(in4 == out4);

    let mut in5: [i32; 5] = [1002,4,3,4,33];
    let     out5: [i32; 5] = [1002, 4, 3, 4, 99];
    cpu::runprog(&mut in5);
    assert!(in5 == out5);

    let mut in6: [i32; 11] = [3,9,8,9,10,9,4,9,99,-1,8];
    let mut in7: [i32; 11] = [3,9,7,9,10,9,4,9,99,-1,8];
    let mut in8: [i32; 9] = [3,3,1108,-1,8,3,4,3,99];
    let mut in9: [i32; 9] = [3,3,1107,-1,8,3,4,3,99];
    let mut in10: [i32; 16] = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let mut in11: [i32; 13] = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    let mut in12: [i32; 47] = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    cpu::runprog(&mut in6);
    cpu::runprog(&mut in7);
    cpu::runprog(&mut in8);
    cpu::runprog(&mut in9);
    cpu::runprog(&mut in10);
    cpu::runprog(&mut in11);
    cpu::runprog(&mut in12);

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
    cpu::runprog(&mut input1);
}
