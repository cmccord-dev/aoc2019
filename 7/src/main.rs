use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc;

fn get_param(mem: &Vec<i32>, mode: i32, value: i32) -> i32 {
    match mode {
        0 => mem[value as usize],
        1 => value,
        _ => panic!("invalid mode"),
    }
}

fn run_function(mem: &mut Vec<i32>, rx: mpsc::Receiver<i32>, tx: mpsc::Sender<i32>) {
    //let mut mem = mem.clone();
    let mut pc: usize = 0;
    loop {
        let full_op = mem[pc];
        let op = full_op % 100;
        let c_mode = (full_op / 100) % 10;
        let b_mode = (full_op / 1000) % 10;
        let a_mode = (full_op / 10000) % 10;
        assert!(a_mode == 0);
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
                mem[dst as usize] = rx.recv().unwrap();
                pc = pc + 2;
            }
            4 => {
                //output
                let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                tx.send(arg1);
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
            6 => {
                //bfalse
                let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                let arg2 = get_param(&mem, b_mode, mem[pc + 2]);
                if arg1 == 0 {
                    pc = arg2 as usize;
                } else {
                    pc = pc + 3;
                }
            }
            7 => {
                //slt
                let arg1 = get_param(&mem, c_mode, mem[pc + 1]);
                let arg2 = get_param(&mem, b_mode, mem[pc + 2]);
                let dst = mem[pc + 3];
                mem[dst as usize] = if arg1 < arg2 { 1 } else { 0 };
                pc = pc + 4;
            }
            8 => {
                //seq
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
}

fn test_input(mem: &Vec<i32>, initial: &Vec<i32>) -> Result<i32, Box<std::error::Error>> {
    //let mut output = 0;
    let mem_a = mem.clone();
    let mem_b = mem.clone();
    let mem_c = mem.clone();
    let mem_d = mem.clone();
    let mem_e = mem.clone();
    let (tx_in, rx_a) = mpsc::channel::<i32>();
    let (tx_a, rx_b) = mpsc::channel::<i32>();
    let (tx_b, rx_c) = mpsc::channel::<i32>();
    let (tx_c, rx_d) = mpsc::channel::<i32>();
    let (tx_d, rx_e) = mpsc::channel::<i32>();
    let (tx_e, rx_out) = mpsc::channel::<i32>();
    tx_in.send(initial[0])?;
    tx_a.send(initial[1])?;
    tx_b.send(initial[2])?;
    tx_c.send(initial[3])?;
    tx_d.send(initial[4])?;
    let a = std::thread::spawn(move || {
        run_function(&mut mem_a.clone(), rx_a, tx_a);
    });
    let b = std::thread::spawn(move || {
        run_function(&mut mem_b.clone(), rx_b, tx_b);
    });
    let c = std::thread::spawn(move || {
        run_function(&mut mem_c.clone(), rx_c, tx_c);
    });
    let d = std::thread::spawn(move || {
        run_function(&mut mem_d.clone(), rx_d, tx_d);
    });
    let e = std::thread::spawn(move || {
        run_function(&mut mem_e.clone(), rx_e, tx_e);
    });
    let output = std::thread::spawn(move || {});
    tx_in.send(0)?;
    a.join();
    b.join();
    c.join();
    d.join();
    e.join();
    output.join();
    match rx_out.recv() {
        Ok(val) => Ok(val),
        Err(err) => Err(Box::new(err)),
    }
}

fn test_input2(mem: &Vec<i32>, initial: &Vec<i32>) -> Result<i32, Box<std::error::Error>> {
    //let mut output = 0;
    let mem_a = mem.clone();
    let mem_b = mem.clone();
    let mem_c = mem.clone();
    let mem_d = mem.clone();
    let mem_e = mem.clone();
    let (tx_in, rx_a) = mpsc::channel::<i32>();
    let (tx_a, rx_b) = mpsc::channel::<i32>();
    let (tx_b, rx_c) = mpsc::channel::<i32>();
    let (tx_c, rx_d) = mpsc::channel::<i32>();
    let (tx_d, rx_e) = mpsc::channel::<i32>();
    let (tx_e, rx_out) = mpsc::channel::<i32>();
    tx_in.send(initial[0])?;
    tx_a.send(initial[1])?;
    tx_b.send(initial[2])?;
    tx_c.send(initial[3])?;
    tx_d.send(initial[4])?;
    let a = std::thread::spawn(move || {
        run_function(&mut mem_a.clone(), rx_a, tx_a);
    });
    let b = std::thread::spawn(move || {
        run_function(&mut mem_b.clone(), rx_b, tx_b);
    });
    let c = std::thread::spawn(move || {
        run_function(&mut mem_c.clone(), rx_c, tx_c);
    });
    let d = std::thread::spawn(move || {
        run_function(&mut mem_d.clone(), rx_d, tx_d);
    });
    let e = std::thread::spawn(move || {
        run_function(&mut mem_e.clone(), rx_e, tx_e);
    });
    tx_in.send(0)?;
    let output = std::thread::spawn(move || {
        let mut last = 0;
        while let Ok(val) = rx_out.recv() {
            last = val;
            //dbg!(last);
            tx_in.send(last);
        }
        last
    });
    a.join();
    b.join();
    c.join();
    d.join();
    e.join();
    Ok(output.join().unwrap())
}
fn main() {
    //let mut input: Vec<i32> = vec![5];
    match get_input::<i32>("input.test") {
        Ok(mem) => {
            //let mut indices = vec![0, 1, 2, 3, 4];
            let mut indices = vec![5, 6, 7, 8, 9];
            let mut c = (0..5).map(|_| 0).collect::<Vec<usize>>();
            let n = 5;
            let mut i = 0;
            let mut count = 0;
            //dbg!(test_input2(&mem, &vec!(9, 8, 7, 6, 5)).unwrap());
            let mut max = test_input2(&mem, &indices).unwrap();
            while i < n {
                if c[i] < i {
                    if i & 1 == 0 {
                        indices.swap(0, i);
                    } else {
                        indices.swap(c[i], i);
                    }
                    let val = test_input2(&mem, &indices).unwrap();
                    if val > max {
                        max = val;
                    }
                    count = count + 1;
                    c[i] += 1;
                    i = 0;
                } else {
                    c[i] = 0;
                    i += 1;
                }
            }
            dbg!(max);
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
