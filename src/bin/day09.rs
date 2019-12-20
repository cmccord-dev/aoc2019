use aoc2019::{get_input, Intcode, Interrupt};

fn run(val: i64) -> i64 {
    let mut int = Intcode::new(get_input::<i64>("input/day09.in").unwrap());
    int.write_input(val);
    match int.run_until_interrupt() {
        Interrupt::Output => int.read_output().unwrap(),
        _ => panic!("Expected output"),
    }
}

fn main() {
    println!("Part 1: {}", run(1));
    println!("Part 2: {}", run(2));
}
