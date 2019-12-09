extern crate nalgebra as na;
use na::Vector2;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type Vec2 = Vector2<i32>;

struct LineSegment {
    pub min: Vec2,
    pub max: Vec2,
    pub start: Vec2,
    pub horizontal: bool,
}
impl std::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Seg ({}, {}) -> ({}, {}) {}",
            self.min.x,
            self.min.y,
            self.max.x,
            self.max.y,
            if self.horizontal { "H" } else { "V" }
        )
    }
}
impl LineSegment {
    pub fn new(a: &Vec2, b: &Vec2) -> LineSegment {
        match (a.x == b.x, a.y == b.y, a.y > b.y, a.x > b.x) {
            (true, _, true, _) => LineSegment {
                min: b.clone(),
                max: a.clone(),
                start: a.clone(),
                horizontal: false,
            },
            (_, true, _, true) => LineSegment {
                min: b.clone(),
                max: a.clone(),
                start: a.clone(),
                horizontal: true,
            },
            (_, horizontal, _, _) => LineSegment {
                min: a.clone(),
                max: b.clone(),
                start: a.clone(),
                horizontal: horizontal,
            },
        }
    }
    pub fn intersect(a: &LineSegment, b: &LineSegment) -> Option<Vec2> {
        let res = match (a.horizontal, b.horizontal) {
            (false, false) => None,
            (true, true) => None,
            (false, true) => {
                let x = a.min.x;
                let y = b.min.y;
                if (b.min.x..b.max.x).contains(&x) && (a.min.y..a.max.y).contains(&y) {
                    Some(Vec2::new(x, y))
                } else {
                    None
                }
            }
            (true, false) => {
                let x = b.min.x;
                let y = a.min.y;
                if (a.min.x..a.max.x).contains(&x) && (b.min.y..b.max.y).contains(&y) {
                    Some(Vec2::new(x, y))
                } else {
                    None
                }
            }
        };
        if a.horizontal != b.horizontal {
            /*dbg!(a);
            dbg!(b);
            dbg!(res);*/
        }
        res
    }
    pub fn len(&self) -> i32 {
        (self.max - self.min).abs().max()
    }
}

fn manhattan(a: &Vec2) -> i32 {
    a.x.abs() + a.y.abs()
}
fn get_input(name: &str) -> Result<(Vec<LineSegment>, Vec<LineSegment>), std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    println!("Parsing input");
    let mut res: Vec<Vec<LineSegment>> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut vals: Vec<LineSegment> = Vec::new();
            let mut start = Vec2::new(0, 0);
            for part in line.split(',') {
                //println!("{:?}", part);
                let dir = part.chars().nth(0).unwrap();
                let dist = part[1..].parse::<i32>().unwrap();
                let off = match dir {
                    'R' => Vec2::new(dist, 0),
                    'L' => Vec2::new(-dist, 0),
                    'U' => Vec2::new(0, dist),
                    'D' => Vec2::new(0, -dist),
                    _ => Vec2::new(0, 0),
                };
                vals.push(LineSegment::new(&start, &(start + off)));
                start = start + off;
            }
            vals
        })
        .collect::<Vec<Vec<LineSegment>>>();
    let a = res.pop().unwrap();
    let b = res.pop().unwrap();
    Ok((b, a))
}
fn main() {
    let (a, b) = get_input("../part1.in").ok().unwrap();
    /* //part 1
    let mut min = std::i32::MAX;
    for seg1 in a.iter() {
        for seg2 in b.iter() {
            if let Some(point) = LineSegment::intersect(seg1, seg2) {
                let val = manhattan(&point);
                if val != 0 {
                    //println!("{}", val);
                    if val < min {
                        min = val;
                    }
                }
            }
        }
    }
    println!("{}", min);*/
    let mut steps_a = 0;
    let mut min = std::i32::MAX;
    for seg1 in a.iter() {
        let mut steps_b = 0;
        for seg2 in b.iter() {
            if let Some(point) = LineSegment::intersect(seg1, seg2) {
                if manhattan(&point) != 0 {
                    let pa = LineSegment::new(&seg1.start, &point);
                    let pb = LineSegment::new(&seg2.start, &point);
                    let dist = steps_a + pa.len() + steps_b + pb.len();
                    if dist < min {
                        min = dist;
                    }
                }
            }
            //dbg!((seg2, seg2.len()));
            steps_b += seg2.len();
        }
        steps_a += seg1.len();
    }
    println!("{}", min);
}
