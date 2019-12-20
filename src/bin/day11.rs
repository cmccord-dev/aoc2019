use aoc2019::{get_input, Intcode, IntcodeIO};
use std::collections::HashMap;

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
    fn print_pattern(&self) -> String {
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
            .collect::<Vec<String>>()
            .join("\n")
    }
}
impl IntcodeIO for PaintingBotIO {
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

fn run(val: i64) -> PaintingBotIO {
    let mut int = Intcode::new(get_input::<i64>("input/day11.in").unwrap());
    let mut paintingbot = PaintingBotIO::new();
    paintingbot.map.insert((0, 0), val);
    int.run_with_io(&mut paintingbot);
    paintingbot
}

fn main() {
    println!("Part 1: {}", run(0).map.keys().len());
    println!("Part 2: \n{}", run(1).print_pattern());
}
