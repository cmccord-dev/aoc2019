use std::collections::HashMap;
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
fn get_param_dst(_mem: &Vec<i64>, mode: i64, rb: i64, value: i64) -> usize {
    match mode {
        0 => value as usize,
        1 => panic!("not supported"),
        2 => (value + rb) as usize,
        _ => panic!("invalid mode"),
    }
}

pub enum CmdState {
    X,
    Y,
    TileID,
}
impl CmdState {
    fn next(&self) -> CmdState {
        match self {
            CmdState::X => CmdState::Y,
            CmdState::Y => CmdState::TileID,
            CmdState::TileID => CmdState::X,
        }
    }
}

trait IO {
    fn read(&self) -> i64;
    fn write(&mut self, data: i64);
}
struct ArcadeIO {
    map: HashMap<(i32, i32), i64>,
    state: CmdState,
    x: i32,
    y: i32,
    score: i64,
    ball_loc: (i32, i32),
    paddle_loc: (i32, i32),
}
impl ArcadeIO {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            state: CmdState::X,
            x: 0,
            y: 0,
            ball_loc: (-1, -1),
            paddle_loc: (-1, -1),
            score: 0,
        }
    }
    fn print_map(&self) {
        let x_size = 41;
        let y_size = 19;
        for y in 0..=y_size {
            for x in 0..=x_size {
                print!(
                    "{}",
                    match self.map.get(&(x, y)) {
                        Some(0) => ' ',
                        Some(1) => '#',
                        Some(2) => 'X',
                        Some(3) => '_',
                        Some(4) => 'o',
                        _ => panic!("unknown type at {} {}", x, y),
                    }
                );
            }
            println!();
        }
    }
}
impl IO for ArcadeIO {
    fn read(&self) -> i64 {
        //self.print_map();
        match self.ball_loc.0.cmp(&self.paddle_loc.0) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
        }
    }
    fn write(&mut self, data: i64) {
        match self.state {
            CmdState::X => self.x = data as i32,
            CmdState::Y => self.y = data as i32,
            CmdState::TileID => {
                if self.x == -1 && self.y == 0 {
                    self.score = data;
                } else {
                    self.map.insert((self.x, self.y), data);
                    if data == 4 {
                        self.ball_loc = (self.x, self.y);
                    }
                    if data == 3 {
                        self.paddle_loc = (self.x, self.y);
                    }
                }
            }
        }
        self.state = self.state.next();
    }
}

fn run_function(mem: &mut Vec<i64>, io: &mut IO) {
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
                mem[dst] = io.read();
                pc = pc + 2;
            }
            4 => {
                //output
                let arg1 = get_param(&mem, c_mode, rb, mem[pc + 1]);
                io.write(arg1);
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

fn test_input(mem: &Vec<i64>) {
    let mem_a = mem.clone();
    let (tx_in, rx_a) = mpsc::channel::<i64>();
    let (tx_a, rx_out) = mpsc::channel::<i64>();
    let mut arcade = ArcadeIO::new();
    run_function(&mut mem_a.clone(), &mut arcade);
    println!(
        "Total blocks: {}",
        arcade.map.values().filter(|v| **v == 2).count()
    );
    let mut mem_a = mem.clone();
    let mut arcade = ArcadeIO::new();
    mem_a[0] = 2;
    run_function(&mut mem_a.clone(), &mut arcade);
    arcade.print_map();
    println!("Final score: {}", arcade.score);
}

fn main() {
    //let mut input: Vec<i64> = vec![5];
    match get_input::<i64>("input.tx") {
        Ok(mem) => {
            test_input(&mem);
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
