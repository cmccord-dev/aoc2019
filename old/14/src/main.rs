use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Recipe {
    result: (i64, String),
    ingredients: Vec<(i64, String)>,
    edges: Vec<String>,
}

impl std::str::FromStr for Recipe {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" => ").collect::<Vec<&str>>();
        let ingredients = s[0]
            .split(", ")
            .map(|a| {
                let tmp = a.split(" ").collect::<Vec<&str>>();
                (tmp[0].parse::<i64>().unwrap(), String::from(tmp[1]))
            })
            .collect::<Vec<(i64, String)>>();
        let tmp = s[1].split(" ").collect::<Vec<&str>>();
        let result = (tmp[0].parse::<i64>()?, String::from(tmp[1]));
        Ok(Self {
            result,
            edges: Vec::new(),
            ingredients,
        })
    }
}

fn get_input(name: &str, split: char) -> Result<Vec<Recipe>, std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input
        .split(split)
        .filter(|line| line.len() > 0)
        .map(|a| a.parse().unwrap())
        .collect::<Vec<Recipe>>())
}

fn setup() -> (
    HashMap<String, Recipe>,
    HashMap<String, i64>,
    HashMap<String, i64>,
    VecDeque<String>,
) {
    let input = get_input("input.txt", '\n').unwrap();
    let mut map: HashMap<String, Recipe> = HashMap::new();
    let mut needed: HashMap<String, i64> = HashMap::new();
    let mut have: HashMap<String, i64> = HashMap::new();
    input.into_iter().for_each(|recipe| {
        have.insert(recipe.result.1.clone(), 0);
        needed.insert(recipe.result.1.clone(), 0);
        if let Some(r) = map.insert(recipe.result.1.clone(), recipe) {
            panic!("Did not expect: {:?}", r);
        }
    });

    let all = map.keys().map(|a| a.clone()).collect::<Vec<String>>();
    all.iter().for_each(|k| {
        let children = map[k]
            .ingredients
            .iter()
            .map(|a| a.1.clone())
            .collect::<Vec<String>>();
        children.iter().for_each(|c| {
            if c != "ORE" {
                map.get_mut(c).unwrap().edges.push(k.clone());
            }
        })
    });

    //topological sort :D
    let mut order: VecDeque<String> = VecDeque::new();
    let mut S: VecDeque<String> = VecDeque::new();
    S.push_back(String::from("FUEL"));

    while S.len() > 0 {
        let n = S.pop_front().unwrap();
        order.push_back(n.clone());
        let children = map[&n]
            .ingredients
            .iter()
            .filter(|a| a.1 != "ORE")
            .map(|a| a.1.clone())
            .collect::<Vec<String>>();
        children.iter().for_each(|child| {
            let child = map.get_mut(child).unwrap();
            if let Some(index) = child.edges.iter().position(|v| v == &n) {
                child.edges.swap_remove(index);
                if child.edges.len() == 0 {
                    S.push_back(child.result.1.clone());
                }
            }
        });
    }
    (map, needed, have, order)
}

fn run(
    amount: i64,
    mut needed: HashMap<String, i64>,
    mut have: HashMap<String, i64>,
    map: &HashMap<String, Recipe>,
    order: &VecDeque<String>,
) -> (i64, HashMap<String, i64>, HashMap<String, i64>) {
    let mut ore_needed = 0;
    needed.insert(String::from("FUEL"), amount);
    order.iter().for_each(|need| {
        let needed_qty = needed[need];
        let recipe = &map[need];
        let output_qty = recipe.result.0;
        let batches = (needed_qty + (output_qty - 1)) / output_qty;

        recipe.ingredients.iter().for_each(|ingr| {
            if ingr.1 == "ORE" {
                ore_needed += (ingr.0 * batches) as i64;
            } else {
                let curr = have.get_mut(&ingr.1).unwrap();
                let need = needed.get_mut(&ingr.1).unwrap();
                *need += ingr.0 * batches;
                if *curr <= *need {
                    *need -= *curr;
                    *curr = 0;
                } else if *curr > *need {
                    *curr -= *need;
                    *need = 0;
                }
            }
        });
        *have.get_mut(&recipe.result.1).unwrap() = batches * output_qty - needed_qty;
        *needed.get_mut(need).unwrap() = 0;
    });
    (ore_needed, have, needed)
}

fn part_1() -> i64 {
    let (map, needed, have, order) = setup();
    let (ore, _, _) = run(1, needed, have, &map, &order);
    ore
}

fn part_2() -> i64 {
    let (map, mut needed, mut have, order) = setup();
    //needed.insert((String::from("ORE"), 0);
    let mut ore_needed: i64 = 0;
    let mut fuel: i64 = 0;
    let mut req = 1000000000000 / (part_1() as i64);
    loop {
        let (ore, have_tmp, need_tmp) = run(req, needed.clone(), have.clone(), &map, &order);
        if ore_needed + ore < 1000000000000 {
            fuel += req;
            ore_needed += ore;
            have = have_tmp;
            needed = need_tmp;
        } else {
            if req == 1 {
                break;
            }
            req /= 2;
        }
    }
    fuel
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}