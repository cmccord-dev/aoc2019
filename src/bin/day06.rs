use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Body {
    name: String,
    value: Option<i32>,
    parent: Option<usize>,
    children: Vec<usize>,
}
impl Body {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            value: None,
            parent: None,
            children: Vec::new(),
        }
    }
}

fn find_value(all_bodies: &mut Vec<Body>, node: usize) -> i32 {
    if let Some(val) = all_bodies[node].value {
        val
    } else if let Some(parent) = all_bodies[node].parent {
        let val = 1 + find_value(all_bodies, parent);
        all_bodies[node].value = Some(val);
        val
    } else {
        0
    }
}
fn main() {
    let input = get_input::<Orbit>("input/day06.in", '\n').ok().unwrap();
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut all_bodies = Vec::new();
    let mut you = std::usize::MAX;
    let mut san = std::usize::MAX;
    input.iter().for_each(|orbit| {
        if !map.contains_key(&orbit.orbiter) {
            all_bodies.push(Body::new(&orbit.orbiter));
            map.insert(orbit.orbiter.clone(), all_bodies.len() - 1);
            if orbit.orbiter == "YOU" {
                you = all_bodies.len() - 1;
            }
            if orbit.orbiter == "SAN" {
                san = all_bodies.len() - 1;
            }
        }
        if !map.contains_key(&orbit.orbitee) {
            all_bodies.push(Body::new(&orbit.orbitee));
            map.insert(orbit.orbitee.clone(), all_bodies.len() - 1);
            if orbit.orbitee == "YOU" {
                you = all_bodies.len() - 1;
            }
            if orbit.orbitee == "SAN" {
                san = all_bodies.len() - 1;
            }
        }
        all_bodies[map[&orbit.orbiter]]
            .children
            .push(map[&orbit.orbitee]);
        all_bodies[map[&orbit.orbitee]].parent = Some(map[&orbit.orbiter]);
    });

    //find value
    /*(0..all_bodies.len())
    .map(|i| find_value(&mut all_bodies, i))
    .sum::<i32>();*/
    let mut q: Vec<usize> = vec![you];
    all_bodies[you].value = Some(0);
    while q.len() > 0 {
        let curr = q.pop().unwrap();
        if all_bodies[curr].name == "SAN" {
            dbg!(&all_bodies[curr].value);
            break;
        }
        if let Some(parent) = all_bodies[curr].parent {
            if let None = all_bodies[parent].value {
                all_bodies[parent].value = Some(all_bodies[curr].value.unwrap() + 1);
                q.push(parent);
            }
        }
        let children = all_bodies[curr].children.clone();
        children.iter().for_each(|child| {
            if let None = all_bodies[*child].value {
                all_bodies[*child].value = Some(all_bodies[curr].value.unwrap() + 1);
                q.push(*child);
            }
        });
    }
}

#[derive(Debug)]
struct Orbit {
    orbiter: String,
    orbitee: String,
}

impl std::str::FromStr for Orbit {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<&str> = s.split(')').collect();

        let orbiter = String::from(pair[0]);
        let orbitee = String::from(pair[1]);
        Ok(Orbit { orbiter, orbitee })
    }
}

fn get_input<T: std::str::FromStr>(name: &str, split: char) -> Result<Vec<T>, std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    println!("Parsing input");
    Ok(input
        .split(split)
        .filter(|line| line.len() > 0)
        .map(|a| a.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>())
}
