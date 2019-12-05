use std::fs;
use std::env;

// This is terrible
// made everything mutable since I need to sleep soon
// I'd prefer to copy the array on function call and return a new instance
// fix later, need to learn this stuff properly

//fn runprog (program: [u32]) -> &[u32]{
fn runprog (program: &mut [u32]) {
    // start at location 0
    let mut pc = 0;

    // loop until program finish
    loop {
        if program[pc] == 1 {
            program[program[pc+3] as usize] = program[program[pc+1] as usize] + program[program[pc+2] as usize];
            pc += 4;
        } else if program[pc] == 2 {
            program[program[pc+3] as usize] = program[program[pc+1] as usize] * program[program[pc+2] as usize];
            pc += 4;
        } else if program[pc] == 99 {
            break
        } else {
            println!("Found incorrect opcode {} on location {}", program[pc], pc);
        }
    }
}


fn main() {
    println!("Validating algorithm");
    let mut in1: [u32; 5] = [1,0,0,0,99];
    let     out1: [u32; 5] = [2,0,0,0,99];
    let mut in2: [u32; 5] = [2,3,0,3,99];
    let     out2: [u32; 5] = [2,3,0,6,99];
    let mut in3: [u32; 6] = [2,4,4,5,99,0];
    let     out3: [u32; 6] = [2,4,4,5,99,9801];
    let mut in4: [u32; 9] = [1,1,1,4,99,5,6,0,99];
    let     out4: [u32; 9] = [30,1,1,4,2,5,6,0,99];

    runprog(&mut in1);
    runprog(&mut in2);
    runprog(&mut in3);
    runprog(&mut in4);

    assert!(in1 == out1);
    assert!(in2 == out2);
    assert!(in3 == out3);
    assert!(in4 == out4);

    // Read file location from stdin
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Opening file {}", filename);
    let prog = fs::read_to_string(filename)
        .expect("Cannot open file");

    //println!("{:?}", prog);
    let input_orig: Vec<u32> = prog
        .trim() // remove last \n
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    //println!("{:?}", input);

    let mut input1 = input_orig.clone();
    // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    input1[1] = 12;
    input1[2] = 2;
    runprog(&mut input1);
    println!("part 1: {}", input1[0]);
    assert!(input1 != input_orig);

    //find two values that result in a specific value
    const WANTED: u32 =  19690720;

    let mut noun = 0;
    let mut verb = 0;
    let mut found = false;

    // brute force search - since we don't know what the program is doing, can't optimise search
    // also its late and with only 10k values, this should be fast
    while noun < 100 {
        verb = 0;
        while verb < 100 {
            let mut input = input_orig.clone();
            input[1] = noun;
            input[2] = verb;
            runprog(&mut input);
            if input[0] == WANTED {
                found = true;
                break;
            }
            else {
                verb += 1;
            }
        }
        if found {
            break;
        }
        else {
            noun += 1;
        }
    }

    println!("Part 2: {}", 100 * noun + verb);
}
