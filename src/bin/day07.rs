use aoc2019::{get_input, Intcode, Interrupt};

fn trial(input: Vec<i64>) -> i64 {
    let mem = get_input::<i64>("input/day07.in").unwrap();
    let mut thrusters = (0..5)
        .map(|_| Intcode::new(mem.clone()))
        .collect::<Vec<Intcode>>();
    (0..5).for_each(|i| thrusters[i].write_input(input[i]));
    thrusters[0].write_input(0);
    let mut curr = 0;
    let mut next = 1;
    let mut val = 0;
    loop {
        match thrusters[curr].run_until_interrupt() {
            Interrupt::Input => panic!("Didn't expect input"),
            Interrupt::Halt => {
                curr = (curr + 1) % 5;
                next = (next + 1) % 5;
                if thrusters.iter().all(|t| t.is_halted()) {
                    break;
                }
            }
            Interrupt::Output => {
                val = thrusters[curr].read_output().unwrap();
                thrusters[next].write_input(val);
                curr = (curr + 1) % 5;
                next = (next + 1) % 5;
            }
        }
    }
    val
}

fn part1() -> i64 {
    let mut data: Vec<i64> = (0..5).collect();
    permutohedron::Heap::new(&mut data)
        .map(trial)
        .max()
        .unwrap()
}
fn part2() -> i64 {
    let mut data: Vec<i64> = (5..10).collect();
    permutohedron::Heap::new(&mut data)
        .map(trial)
        .max()
        .unwrap()
}
fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
