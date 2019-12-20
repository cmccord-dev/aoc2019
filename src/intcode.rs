use crate::IntcodeIO;
use std::collections::VecDeque;

pub struct Intcode {
    mem: Vec<i64>,
    pc: usize,
    rb: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    interrupt: Option<Interrupt>,
    halted: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    Input,
    Halt,
    Output,
}
enum ParamMode {
    Absolute,
    Immediate,
    Relative,
}

impl From<i64> for ParamMode {
    fn from(val: i64) -> ParamMode {
        match val {
            0 => ParamMode::Absolute,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unknown parameter mode {}"),
        }
    }
}

struct Instr {
    op: i64,
    a: ParamMode,
    b: ParamMode,
    c: ParamMode,
}
impl From<i64> for Instr {
    fn from(val: i64) -> Instr {
        Instr {
            op: val % 100,
            c: ParamMode::from((val / 100) % 10),
            b: ParamMode::from((val / 1000) % 10),
            a: ParamMode::from((val / 10000) % 10),
        }
    }
}

impl Intcode {
    pub fn new(mut mem: Vec<i64>) -> Self {
        mem.resize(0x1000, 0);
        Self {
            mem,
            pc: 0,
            rb: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            interrupt: None,
            halted: true,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }
    pub fn run_until_interrupt(&mut self) -> Interrupt {
        loop {
            self.step();
            if let Some(int) = self.interrupt {
                self.interrupt = None;
                break int;
            }
        }
    }

    pub fn run_with_io<T: IntcodeIO>(&mut self, io: &mut T) {
        loop {
            self.step();
            match self.interrupt {
                Some(Interrupt::Halt) => break,
                Some(Interrupt::Input) => {
                    self.write_input(io.read());
                    self.interrupt = None;
                }
                Some(Interrupt::Output) => {
                    io.write(self.read_output().unwrap());
                    self.interrupt = None;
                }
                None => (),
            }
        }
    }

    pub fn write_input(&mut self, val: i64) {
        self.input.push_back(val);
    }
    pub fn read_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn write_mem(&mut self, addr: usize, val: i64) {
        self.mem[addr] = val;
    }
    pub fn read_mem(&self, addr: usize) -> i64 {
        self.mem[addr]
    }

    fn fetch_instr(&mut self) -> Instr {
        self.pc += 1;
        self.read_mem(self.pc - 1).into()
    }

    fn read_param(&mut self, mode: ParamMode) -> i64 {
        let val = self.read_mem(self.pc);
        self.pc += 1;
        match mode {
            ParamMode::Absolute => self.read_mem(val as usize),
            ParamMode::Immediate => val,
            ParamMode::Relative => self.read_mem((self.rb + val) as usize),
        }
    }
    fn write_param(&mut self, value: i64, mode: ParamMode) {
        let val = self.read_mem(self.pc);
        self.pc += 1;
        match mode {
            ParamMode::Absolute => self.write_mem(val as usize, value),
            ParamMode::Immediate => panic!("Can't write immediate mode"),
            ParamMode::Relative => self.write_mem((self.rb + val) as usize, value),
        }
    }

    pub fn step(&mut self) {
        let instr = self.fetch_instr();
        match instr.op {
            1 => {
                //add
                let c = self.read_param(instr.c);
                let b = self.read_param(instr.b);
                self.write_param(c + b, instr.a);
            }
            2 => {
                //mul
                let c = self.read_param(instr.c);
                let b = self.read_param(instr.b);
                self.write_param(c * b, instr.a);
            }
            3 => {
                //input
                match self.input.pop_front() {
                    Some(val) => self.write_param(val, instr.c),
                    None => {
                        self.pc -= 1;
                        self.interrupt = Some(Interrupt::Input);
                    }
                }
            }
            4 => {
                //output
                let c = self.read_param(instr.c);
                self.output.push_back(c);
                self.interrupt = Some(Interrupt::Output);
            }
            5 => {
                //b true
                if self.read_param(instr.c) != 0 {
                    self.pc = self.read_param(instr.b) as usize;
                } else {
                    self.pc += 1;
                }
            }
            6 => {
                //bfalse
                if self.read_param(instr.c) == 0 {
                    self.pc = self.read_param(instr.b) as usize;
                } else {
                    self.pc += 1;
                }
            }
            7 => {
                //slt
                let c = self.read_param(instr.c);
                let b = self.read_param(instr.b);
                self.write_param(if c < b { 1 } else { 0 }, instr.a);
            }
            8 => {
                //seq
                let c = self.read_param(instr.c);
                let b = self.read_param(instr.b);
                self.write_param(if c == b { 1 } else { 0 }, instr.a);
            }
            9 => {
                self.rb += self.read_param(instr.c);
            }
            99 => {
                self.interrupt = Some(Interrupt::Halt);
                self.halted = true;
            }
            _ => panic!(
                "bad instruction at {}, {}",
                self.pc - 1,
                self.read_mem(self.pc - 1)
            ),
        }
    }
}
