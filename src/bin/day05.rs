use aoc2019::get_input;
use aoc2019::{Intcode, Interrupt};

fn run_intcode_with_input(val: i64) -> i64 {
    let mut int = Intcode::new(get_input::<i64>("input/day05.in").unwrap());
    int.write_input(val);
    let mut last = 0;
    loop {
        match int.run_until_interrupt() {
            Interrupt::Output => last = int.read_output().unwrap(),
            Interrupt::Halt => break last,
            _ => panic!("Input unexpected"),
        }
    }
}
fn part_1() -> i64 {
    run_intcode_with_input(1)
}
fn part_2() -> i64 {
    run_intcode_with_input(5)
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
