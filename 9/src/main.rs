use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc;

fn get_param(mem: &Vec<i64>, mode: i64, rb: i64, value: i64) -> i64 {
    match mode {
        0 => mem[value as usize],
        1 => value,
        2 => mem[(value + rb) as usize],
        _ => panic!("invalid mode"),
    }
}
fn get_param_dst(mem: &Vec<i64>, mode: i64, rb: i64, value: i64) -> usize {
    match mode {
        0 => value as usize,
        1 => panic!("not supported"),
        2 => (value + rb) as usize,
        _ => panic!("invalid mode"),
    }
}

fn run_function(mem: &mut Vec<i64>, rx: mpsc::Receiver<i64>, tx: mpsc::Sender<i64>) {
    //let mut mem = mem.clone();
    mem.resize(0x1000, 0);
    let mut pc: usize = 0;
    let mut rb: i64 = 0;
    loop {
        let full_op = mem[pc];
        let op = full_op % 100;
        let c_mode = (full_op / 100) % 10;
        let b_mode = (full_op / 1000) % 10;
        let a_mode = (full_op / 10000) % 10;
        assert!(a_mode != 1);
        match op {
            1 => {
                //add
                let (arg1, arg2, dst) = (
                    get_param(&mem, c_mode, rb, mem[pc + 1]),
                    get_param(&mem, b_mode, rb, mem[pc + 2]),
                    get_param_dst(&mem, a_mode, rb, mem[pc + 3]),
                );
                mem[dst as usize] = arg1 + arg2;
                pc = pc + 4;
            }
            2 => {
                //mul
                let (arg1, arg2, dst) = (
                    get_param(&mem, c_mode, rb, mem[pc + 1]),
                    get_param(&mem, b_mode, rb, mem[pc + 2]),
                    get_param_dst(&mem, a_mode, rb, mem[pc + 3]),
                );
                mem[dst as usize] = arg1 * arg2;
                pc = pc + 4;
            }
            3 => {
                //input
                let (dst) = get_param_dst(&mem, c_mode, rb, mem[pc + 1]);
                mem[dst] = rx.recv().unwrap();
                pc = pc + 2;
            }
            4 => {
                //output
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                tx.send(arg1);
                pc = pc + 2;
            }
            5 => {
                //b true
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                let arg2 = get_param(&mem, b_mode, rb, mem[pc + 2]);
                if arg1 != 0 {
                    pc = arg2 as usize;
                } else {
                    pc = pc + 3;
                }
            }
            6 => {
                //bfalse
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                let arg2 = get_param(&mem, b_mode, rb, mem[pc + 2]);
                if arg1 == 0 {
                    pc = arg2 as usize;
                } else {
                    pc = pc + 3;
                }
            }
            7 => {
                //slt
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                let arg2 = get_param(&mem, b_mode, rb, mem[pc + 2]);
                let dst = get_param_dst(&mem, a_mode, rb, mem[pc + 3]);
                mem[dst as usize] = if arg1 < arg2 { 1 } else { 0 };
                pc = pc + 4;
            }
            8 => {
                //seq
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                let arg2 = get_param(&mem, b_mode, rb, mem[pc + 2]);
                let dst = get_param_dst(&mem, a_mode, rb, mem[pc + 3]);
                mem[dst as usize] = if arg1 == arg2 { 1 } else { 0 };
                pc = pc + 4;
            }
            9 => {
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                rb += arg1;
                pc = pc + 2;
            }
            99 => break,
            _ => panic!("bad instruction at {}, {}", rb, mem[pc]),
        }
    }
}

fn test_input(mem: &Vec<i64>, input: &Vec<i64>) {
    //let mut output = 0;
    let mem_a = mem.clone();
    let (tx_in, rx_a) = mpsc::channel::<i64>();
    let (tx_a, rx_out) = mpsc::channel::<i64>();
    let a = std::thread::spawn(move || {
        run_function(&mut mem_a.clone(), rx_a, tx_a);
    });
    let output = std::thread::spawn(move || {
        while let Ok(val) = rx_out.recv() {
            println!("{}",val);
        }
    });
    let mut input = input.clone();
    let input_t = std::thread::spawn(move || {
        while input.len() > 0 {
            if let Err(err) = tx_in.send(input.pop().unwrap()) {
                break;
            }
        }
    });
    a.join();
    output.join();
    input_t.join();
}

fn main() {
    //let mut input: Vec<i64> = vec![5];
    match get_input::<i64>("input.test") {
        Ok(mem) => {
            test_input(&mem, &vec![2]);
        }
        Err(err) => panic!("Err reading input {:?}", err),
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
