use super::*;

#[test]
fn opc_1() {
    let mut in1: [i32; 5] = [1,0,0,0,99];
    let     out1: [i32; 5] = [2,0,0,0,99];
    runprog(&mut in1);
    assert!(in1 == out1);
}

#[test]
fn opc_2() {
    let mut in2: [i32; 5] = [2,3,0,3,99];
    let     out2: [i32; 5] = [2,3,0,6,99];
    runprog(&mut in2);
    assert!(in2 == out2);
}

#[test]
fn opc_3() {
    let mut in3: [i32; 6] = [2,4,4,5,99,0];
    let     out3: [i32; 6] = [2,4,4,5,99,9801];
    runprog(&mut in3);
    assert!(in3 == out3);
}

#[test]
fn opc_4() {
    let mut in4: [i32; 9] = [1,1,1,4,99,5,6,0,99];
    let     out4: [i32; 9] = [30,1,1,4,2,5,6,0,99];
    runprog(&mut in4);
    assert!(in4 == out4);
}

#[test]
fn opc_5() {
    let mut in5: [i32; 5] = [1002,4,3,4,33];
    let     out5: [i32; 5] = [1002, 4, 3, 4, 99];
    runprog(&mut in5);
    assert!(in5 == out5);
}

#[test]
fn opc_6() {
    let mut in6: [i32; 11] = [3,9,8,9,10,9,4,9,99,-1,8];
    runprog(&mut in6);
}
#[test]
fn opc_7() {
    let mut in7: [i32; 11] = [3,9,7,9,10,9,4,9,99,-1,8];
    runprog(&mut in7);
}
#[test]
fn opc_8() {
    let mut in8: [i32; 9] = [3,3,1108,-1,8,3,4,3,99];
    runprog(&mut in8);
}
#[test]
fn opc_9() {
    let mut in9: [i32; 9] = [3,3,1107,-1,8,3,4,3,99];
    runprog(&mut in9);
}
#[test]
fn opc_10() {
    let mut in10: [i32; 16] = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    runprog(&mut in10);
}
#[test]
fn opc_11() {
    let mut in11: [i32; 13] = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    runprog(&mut in11);
}
#[test]
fn opc_12() {
    let mut in12: [i32; 47] = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    runprog(&mut in12);
}

