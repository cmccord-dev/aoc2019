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

trait IO {
    fn read(&self) -> i64;
    fn write(&mut self, data: i64);
}
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl Dir {
    fn rotate_left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
    fn rotate_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}
enum PaintingBotState {
    Painting,
    Moving,
}
impl PaintingBotState {
    fn next(&self) -> Self {
        match self {
            PaintingBotState::Moving => PaintingBotState::Painting,
            PaintingBotState::Painting => PaintingBotState::Moving,
        }
    }
}
struct PaintingBotIO {
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
    map: HashMap<(i32, i32), i64>,
    dir: Dir,
    state: PaintingBotState,
}
impl PaintingBotIO {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
            map: HashMap::new(),
            dir: Dir::Up,
            state: PaintingBotState::Painting,
        }
    }
    fn move_bot(&mut self) {
        match self.dir {
            Dir::Up => self.y += 1,
            Dir::Right => self.x += 1,
            Dir::Down => self.y -= 1,
            Dir::Left => self.x -= 1,
        }
    }
    fn print_pattern(&self) {
        (self.min_y..self.max_y + 1)
            .map(|y| {
                (self.min_x..self.max_x + 1)
                    .map(|x| match self.map.get(&(x, y)) {
                        Some(1) => '#',
                        _ => ' ',
                    })
                    .collect::<String>()
            })
            .rev()
            .for_each(|line| println!("{}", line))
    }
}
impl IO for PaintingBotIO {
    fn read(&self) -> i64 {
        if !self.map.contains_key(&(self.x, self.y)) {
            0
        } else {
            *self.map.get(&(self.x, self.y)).unwrap()
        }
    }
    fn write(&mut self, data: i64) {
        match self.state {
            PaintingBotState::Painting => {
                assert!(data == 0 || data == 1);
                self.map.insert((self.x, self.y), data);
                self.max_x = self.max_x.max(self.x);
                self.max_y = self.max_y.max(self.y);
                self.min_x = self.min_x.min(self.x);
                self.min_y = self.min_y.min(self.y);
            }
            PaintingBotState::Moving => {
                match data {
                    0 => self.dir = self.dir.rotate_left(),

                    1 => self.dir = self.dir.rotate_right(),

                    _ => panic!("unknown command"),
                };
                self.move_bot()
            }
        };
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
    //let mut output = 0;
    let mem_a = mem.clone();
    let (tx_in, rx_a) = mpsc::channel::<i64>();
    let (tx_a, rx_out) = mpsc::channel::<i64>();
    let mut paintingbot = PaintingBotIO::new();
    //let a = std::thread::spawn(move || {
    paintingbot.map.insert((0, 0), 1);
    run_function(&mut mem_a.clone(), &mut paintingbot);
    println!("{}", paintingbot.map.len());
    paintingbot.print_pattern();
    //});

    /*let output = std::thread::spawn(move || {
        while let Ok(val) = rx_out.recv() {
            println!("{}", val);
        }
    });
    let input_t = std::thread::spawn(move || {
        /*while input.len() > 0 {
            if let Err(err) = tx_in.send(input.pop().unwrap()) {
                break;
            }
        }*/
    });*/
    //a.join();
    //output.join();
    //input_t.join();
}

fn main() {
    //let mut input: Vec<i64> = vec![5];
    match get_input::<i64>("test.input") {
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
