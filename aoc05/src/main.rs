use std::fs;
use std::env;

fn get_opc(instruction: i32) -> u8 {
    return (instruction % 100) as u8;
}

// note: param is 1-indexed
fn get_pmode(instruction: i32, param: u8) -> u8 {
    let mut insc: u32 = (instruction / 100) as u32;
    for _dummy in 1..param {
        insc /= 10;
    }
    return (insc % 10) as u8;
}

fn op01_sum(program: &mut [i32], pc: &mut usize ) {
    let pm1 = get_pmode(program[*pc], 1);
    let pm2 = get_pmode(program[*pc], 2);
    let pm3 = get_pmode(program[*pc], 3);

    let p1:i32 ;
    let p2:i32 ;
    match pm1 {
        0 => p1 = program[program[*pc+1] as usize],
        1 => p1 = program[*pc+1],
        _ => { println!("incorrect pmode {} on location {}", pm1, *pc);
            p1 = 0
        }
    }

    match pm2 {
        0 => p2 = program[program[*pc+2] as usize],
        1 => p2 = program[*pc+2],
        _ => { println!("incorrect pmode {} on location {}", pm1, *pc);
            p2 = 0
        }
    }

    //Parameters that an instruction writes to will never be in immediate mode.
    assert!(pm3 != 1);

    program[program[*pc+3] as usize] = p1 + p2;
    *pc += 4;
}

fn op02_mul(program: &mut [i32], pc: &mut usize) {
    let pm1 = get_pmode(program[*pc], 1);
    let pm2 = get_pmode(program[*pc], 2);
    let pm3 = get_pmode(program[*pc], 3);

    let p1:i32 ;
    let p2:i32 ;
    match pm1 {
        0 => p1 = program[program[*pc+1] as usize],
        1 => p1 = program[*pc+1],
        _ => { println!("incorrect pmode {} on location {}", pm1, *pc);
            p1 = 0
        }
    }

    match pm2 {
        0 => p2 = program[program[*pc+2] as usize],
        1 => p2 = program[*pc+2],
        _ => { println!("incorrect pmode {} on location {}", pm1, *pc);
            p2 = 0
        }
    }

    //Parameters that an instruction writes to will never be in immediate mode.
    assert!(pm3 != 1);

    program[program[*pc+3] as usize] = p1 * p2;
    *pc += 4;
}

fn op03_input(program: &mut [i32], pc: &mut usize) {
    let pm1 = get_pmode(program[*pc], 1);
    //Parameters that an instruction writes to will never be in immediate mode.
    assert!(pm1 != 1);

    // get input
    // for now hardcode to 1
    program[program[*pc+1] as usize] = 1; //FIXME: Take real input

    *pc += 2;
}

fn op04_output(program: &mut [i32], pc: &mut usize) {
    let pm1 = get_pmode(program[*pc], 1);
    let p1:i32 ;
    match pm1 {
        0 => p1 = program[program[*pc+1] as usize],
        1 => p1 = program[*pc+1],
        _ => { println!("incorrect pmode {} on location {}", pm1, *pc);
            p1 = 0
        }
    }
    println!("{}", p1);
    *pc += 2;
}

//change of mindset
//call this function on a mutable intcode array / vector
//and it will execute until finished.
fn runprog (mut program: &mut [i32]) {
    // start at location 0
    let mut pc: usize = 0;

    // loop until program finish
    loop {
        match get_opc(program[pc]) {
            1  => op01_sum(&mut program, &mut pc),
            2  => op02_mul(&mut program, &mut pc),
            3  => op03_input(&mut program, &mut pc),
            4  => op04_output(&mut program, &mut pc),
            99 => break,
            _  => { println!("Found incorrect opcode {} on location {}", program[pc], pc);
                break
            }
        }
    }
}


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

    runprog(&mut in1);
    runprog(&mut in2);
    runprog(&mut in3);
    runprog(&mut in4);

    assert!(in1 == out1);
    assert!(in2 == out2);
    assert!(in3 == out3);
    assert!(in4 == out4);

    let mut in5: [i32; 5] = [1002,4,3,4,33];
    let     out5: [i32; 5] = [1002, 4, 3, 4, 99];
    runprog(&mut in5);
    assert!(in5 == out5);

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
    runprog(&mut input1);
}
