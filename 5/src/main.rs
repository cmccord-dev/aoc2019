use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_param(mem: &Vec<i32>, mode: i32, value: i32) -> i32 {
    match mode {
        0 => mem[value as usize],
        1 => value,
        _ => panic!("invalid mode"),
    }
}
/*fn program_input(input:&mut Vec<i32>) -> i32 {
    input.pop().unwrap_or_else(|| std::io::stdin.)
}*/
fn main() {
    let mut input: Vec<i32> = vec![5];
    if let Ok(mut mem) = get_input::<i32>("input.in") {
        let mut pc: usize = 0;
        loop {
            let full_op = mem[pc];
            let op = full_op % 100;
            let c_mode = (full_op / 100) % 10;
            let b_mode = (full_op / 1000) % 10;
            let a_mode = (full_op / 10000) % 10;
            assert!(a_mode==0);
            match op {
                1 => {
                    //add
                    let (arg1, arg2, dst) = (
                        get_param(&mem, c_mode, mem[pc + 1]),
                        get_param(&mem, b_mode, mem[pc + 2]),
                        mem[pc + 3],
                    );
                    mem[dst as usize] = arg1 + arg2;
                    pc = pc + 4;
                }
                2 => {
                    //mul
                    let (arg1, arg2, dst) = (
                        get_param(&mem, c_mode, mem[pc + 1]),
                        get_param(&mem, b_mode, mem[pc + 2]),
                        mem[pc + 3],
                    );
                    mem[dst as usize] = arg1 * arg2;
                    pc = pc + 4;
                }
                3 => {
                    //input
                    let (dst) = mem[(pc + 1)];
                    mem[dst as usize] = input.pop().unwrap();
                    pc = pc + 2;
                }
                4 => {
                    //output
                    let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                    println!("{}", arg1);
                    pc = pc + 2;
                }
                5 => {
                    //b true
                    let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                    let arg2 = get_param(&mem, b_mode, mem[pc + 2]);
                    if arg1 != 0 {
                        pc = arg2 as usize;
                    } else {
                        pc = pc + 3;
                    }
                }
                6 => { //bfalse
                    let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                    let arg2 = get_param(&mem, b_mode, mem[pc + 2]);
                    if arg1 == 0 {
                        pc = arg2 as usize;
                    } else {
                        pc = pc + 3;
                    }
                }
                7 => { //slt
                    let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                    let arg2 = get_param(&mem, b_mode, mem[pc + 2]);
                    let dst = mem[pc + 3];
                    mem[dst as usize] = if arg1 < arg2 { 1 } else { 0 };
                    pc = pc + 4;
                }
                8 => { //seq
                    let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                    let arg2 = get_param(&mem, b_mode, mem[pc + 2]);
                    let dst = mem[pc + 3];
                    mem[dst as usize] = if arg1 == arg2 { 1 } else { 0 };
                    pc = pc + 4;
                }
                99 => break,
                _ => panic!("bad instruction at {}, {}", pc, mem[pc]),
            }
        }
        println!("{}", mem[0]);
    } else {
        panic!("Err reading input");
    }
}

fn get_input<T: std::str::FromStr>(name: &str) -> Result<Vec<T>, std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    println!("Parsing input");
    Ok(input
        .split(',')
        .filter(|line| line.len() > 0)
        .map(|a| a.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>())
}
