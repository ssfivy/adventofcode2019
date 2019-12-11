use std::convert::TryInto;
//use std::{thread, time};

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

fn op03_input(program: &mut [i32], pc: &mut usize, input: &[i32], inputcount: &mut usize) {
    let pm1 = get_pmode(program[*pc], 1);
    //Parameters that an instruction writes to will never be in immediate mode.
    assert!(pm1 != 1);
    //thread::sleep(time::Duration::from_millis(10));
    println!("{:?}[{:?}]", input[*inputcount], inputcount);

    // get input
    program[program[*pc+1] as usize] = input[*inputcount];

    // increment input array indexing
    // Multiple calls to input opcode will index into this array
    // If there are more calls than array length it will just read the last value
    // I'm sure I won't regret this...
    if *inputcount < input.len()-1 {
        *inputcount += 1;
    }

    *pc += 2;
}

fn op04_output(program: &mut [i32], pc: &mut usize, output: &mut i32) {
    let pm1 = get_pmode(program[*pc], 1);
    let p1:i32 ;
    match pm1 {
        0 => p1 = program[program[*pc+1] as usize],
        1 => p1 = program[*pc+1],
        _ => { println!("incorrect pmode {} on location {}", pm1, *pc);
            p1 = 0
        }
    }
    *output = p1;
    println!("{}", p1);
    *pc += 2;
}

fn op05_jumptrue(program: &mut [i32], pc: &mut usize) {
    let pm1 = get_pmode(program[*pc], 1);
    let pm2 = get_pmode(program[*pc], 2);

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

    if p1 != 0 {
        *pc = p2.try_into().unwrap();
    }
    else {
        *pc += 3;
    }
}

fn op06_jumpfalse(program: &mut [i32], pc: &mut usize) {
    let pm1 = get_pmode(program[*pc], 1);
    let pm2 = get_pmode(program[*pc], 2);

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

    if p1 == 0 {
        *pc = p2.try_into().unwrap();
    }
    else {
        *pc += 3;
    }
}

fn op07_lt(program: &mut [i32], pc: &mut usize ) {
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

    if p1 < p2 {
        program[program[*pc+3] as usize] = 1;
    }
    else {
        program[program[*pc+3] as usize] = 0;
    }

    *pc += 4;
}

fn op08_eq(program: &mut [i32], pc: &mut usize ) {
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

    if p1 == p2 {
        program[program[*pc+3] as usize] = 1;
    }
    else {
        program[program[*pc+3] as usize] = 0;
    }

    *pc += 4;
}


//change of mindset
//call this function on a mutable intcode array / vector
//and it will execute until finished.
pub fn runprog (mut program: &mut [i32],
                mut pc: &mut usize, mut inputcount: &mut usize,
                input: &[i32], mut output: &mut i32) -> bool {

    let mut end: bool = false;

    // loop until program finish
    loop {
        match get_opc(program[*pc]) {
            1  => op01_sum(&mut program, &mut pc),
            2  => op02_mul(&mut program, &mut pc),
            3  => op03_input(&mut program, &mut pc, &input, &mut inputcount),
            4  => {op04_output(&mut program, &mut pc, &mut output); break },
            5  => op05_jumptrue(&mut program, &mut pc),
            6  => op06_jumpfalse(&mut program, &mut pc),
            7  => op07_lt(&mut program, &mut pc),
            8  => op08_eq(&mut program, &mut pc),
            99 => { end = true; break}
            _  => { println!("Found incorrect opcode {} on location {}", program[*pc], pc);
                break
            }
        }
    }

    return end;
}


#[cfg(test)]
mod test;

