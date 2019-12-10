
use itertools::Itertools;

const INPUTLIMITS: [u32; 2] = [248345, 746315];

fn number_to_vec(n: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }
    digits.push(n as u8);
    digits.reverse();
    digits
}

// checks if a password matches the criteria
fn passcheck(pass: u32, part2chk: bool) -> bool {
    /*
    if pass < INPUTLIMITS[0] || pass > INPUTLIMITS[1] {
        // out of range
        return false;
    }
    */

    // convert number to array of digits to check next criterias
    let s = number_to_vec(pass);

    let ss: &Vec<u8> = &s;

    // https://stackoverflow.com/questions/29670170/iterate-over-a-string-n-elements-at-a-time
    // ^^ is not the right answer!
    // and I can't be bothered scanning this list: https://danielkeep.github.io/itercheat_baked.html
    // okay, iterating by index now
    let mut pairfound = false;
    let mut strictincrease = true;
    for i in 0..5 {
        //println!("{:?}",ss[i]);
        // must have at least one pair of adjacent identical digits
        if ss[i] == ss[i+1] {
            pairfound = true;
        }

        //digits never decrease from left to right
        if ss[i] > ss[i+1] {
            strictincrease = false;
        }
    }

    if ! pairfound || ! strictincrease {
        return false;
    }

    // check extra condition for part 2
    if part2chk {
        let mut lastdigit = ss[0];
        let mut identical = 1;
        let mut pairvalid = false;
        for i in 1..6 {
            //println!("i {} ss[i] {} lastdigit {} identical {}", i, ss[i], lastdigit, identical);
            if ss[i] == lastdigit {
                // sequence exists and ongoing
                identical += 1;

                if i == 5 {
                    // sequence ends, analyse it
                    if identical == 2 {
                        pairvalid = true;
                    }
                }
            }
            else {
                // sequence ends, analyse it
                if identical == 2 {
                    pairvalid = true;
                }

                // reset for next sequence
                identical = 1;
            }
            lastdigit = ss[i];
        }

        if ! pairvalid {
            return false;
        }
    }

    //fulfill all criteria, is valid password
    //println!("found: {}",pass);
    return true;
}

fn main() {
    println!("aoc04 start!");

    let mut count: u32 = 0;
    for pass in INPUTLIMITS[0]..INPUTLIMITS[1] {
        if passcheck(pass, false) {
            count += 1;
        }
    }
    println!("Valid password count , part 1: {}", count);

    assert!(passcheck(112233, true));
    assert!(!passcheck(123444, true));
    assert!(passcheck(111122, true));

    let mut count: u32 = 0;
    for pass in INPUTLIMITS[0]..INPUTLIMITS[1] {
        if passcheck(pass, true) {
            count += 1;
        }
    }
    println!("Valid password count , part 2: {}", count);
}
