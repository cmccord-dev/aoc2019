use nalgebra::Vector3;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
type Vec3 = Vector3<i32>;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
        )
    }
}

fn clamp(vec: &Vec3) -> Vec3 {
    Vec3::new(
        nalgebra::clamp(vec.x, -1, 1),
        nalgebra::clamp(vec.y, -1, 1),
        nalgebra::clamp(vec.z, -1, 1),
    )
}

fn step(moons: &mut Vec<Moon>) {
    let grav = moons
        .iter()
        .map(|moon| {
            moons
                .iter()
                .map(move |moon2| {
                    //println!("{}\n{}", moon,moon2);
                    (clamp(&(moon2.pos - moon.pos)))
                })
                .sum()
        })
        .collect::<Vec<Vec3>>();
    moons.iter_mut().enumerate().for_each(|(i, moon)| {
        moon.vel += grav[i];
        moon.pos += moon.vel;
    });
}
fn calculate_potential_energy(moons: &Vec<Moon>) -> i32 {
    moons
        .iter()
        .map(|moon| {
            (moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs())
                * (moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs())
        })
        .sum()
}
fn print_moons(moons: &Vec<Moon>) {
    moons.iter().for_each(|moon| println!("{}", moon))
}
fn main() {
    let mut moons = get_input("input.txt", '\n').unwrap();
    println!("After {} steps", 0);
    print_moons(&moons);
    println!();
    for i in 1..1000 + 1 {
        step(&mut moons);
        //println!("After {} steps", i);
        //print_moons(&moons);
        //println!();
    }
    println!("Total energy = {}", calculate_potential_energy(&moons));

    let mut moons = get_input("input.txt", '\n').unwrap();

    //let mut set: std::collections::HashSet<Vec<Moon>> = std::collections::HashSet::new();
    let mut i: i64 = 0;
    loop {
        step(&mut moons);
        i += 1;
        if moons[0].vel.x == 0
            && moons[0].vel.y == 0
            && moons[0].vel.z == 0
            && moons[1].vel.x == 0
            && moons[1].vel.y == 0
            && moons[1].vel.z == 0
            && moons[2].vel.x == 0
            && moons[2].vel.y == 0
            && moons[2].vel.z == 0
            && moons[3].vel.x == 0
            && moons[3].vel.y == 0
            && moons[3].vel.z == 0
        {
            break;
        }
    }
    println!("{}", i);
}

fn parse_moon(s: &str) -> Moon {
    let pair: Vec<Vec<&str>> = s[1..s.len() - 1]
        .split(',')
        .map(|l| l.split('=').collect())
        .collect();

    Moon {
        pos: Vec3::new(
            pair[0][1].parse::<i32>().unwrap(),
            pair[1][1].parse::<i32>().unwrap(),
            pair[2][1].parse::<i32>().unwrap(),
        ),
        vel: Vec3::new(0, 0, 0),
    }
}

fn get_input(name: &str, split: char) -> Result<Vec<Moon>, std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    println!("Parsing input");
    Ok(input
        .split(split)
        .filter(|line| line.len() > 0)
        .map(|a| parse_moon(a))
        .collect::<Vec<Moon>>())
}
