use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
extern crate nalgebra as na;
use na::distance_squared;
use na::Vector2;
use rayon::prelude::*;
type Vec2 = Vector2<f32>;

fn dist_to_line(v: Vec2, w: Vec2, p0: Vec2) -> f32 {
    //((w.y - v.y) * p0.x - (w.x - v.x) * p0.y + w.x * v.y - w.y * v.x) / ((w - v).norm())
    let l2 = (w - v).norm_squared();
    if l2 < std::f32::EPSILON {
        (p0 - w).norm_squared()
    } else {
        let t = (0.0 as f32).max((1.0 as f32).min((p0 - v).dot(&(w - v)) / l2));
        ((v + t * (w - v)) - p0).norm()
    }
}

fn to_polar(x: &Vec2) -> Vec2 {
    let mut res = Vec2::new(
        (x.x.powf(2.0) + x.y.powf(2.0)).sqrt(),
        x.y.atan2(x.x) + std::f32::consts::FRAC_PI_2,
    );
    if res.y < 0.0 {
        res.y = res.y + std::f32::consts::PI * 2.0;
    }
    res
}
fn to_euclidean(curr: &Vec2) -> Vec2 {
    Vec2::new(
        curr.x * (curr.y - std::f32::consts::FRAC_PI_2).cos(),
        curr.x * (curr.y - std::f32::consts::FRAC_PI_2).sin(),
    )
}

fn main() {
    let input = get_input::<String>("input/day10.in", '\n').unwrap();
    let mut asteroids = Vec::new();
    let mut base: Vec2 = Vec2::new(16.0, 8.0);
    let mut base: Vec2 = Vec2::new(8.0, 16.0);
    input.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                asteroids.push(Vec2::new(j as f32, i as f32));
            }
            if c == 'X' {
                base = Vec2::new(j as f32, i as f32);
            }
        });
    });
    dbg!(base);
    let len = asteroids.len();
    /*let max_vis: (i32, usize) = (0..len).into_par_iter().map(|i| {
        //println!("{}/{}", i, len);
        let a_i = asteroids[i];
        let mut visible = 0;
        for j in 0..len {
            let mut can_see = true;
            let a_j = asteroids[j];
            for k in 0..len {
                if i == j || i == k || j == k {
                    continue;
                }
                let a_k = asteroids[k];
                if dist_to_line(a_i, a_j, a_k) < std::f32::EPSILON {
                    can_see = false;
                }
            }
            if can_see {
                visible = visible + 1;
            }
        }
        (visible, i)
    }).max_by_key(|x| x.0).unwrap();

    dbg!(max_vis.0 - 1);
    dbg!(max_vis.1);
    dbg!(asteroids[max_vis.1]);*/

    asteroids.remove(249);
    let len = len - 1;
    let mut polar = asteroids
        .iter()
        .map(|x| to_polar(&(x - base)))
        .collect::<Vec<Vec2>>();
    polar.sort_by(|x, y| {
        if x.y < y.y {
            std::cmp::Ordering::Less
        } else if x.y == y.y {
            if x.x < y.x {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let mut last_theta: f32 = -1.0;
    let mut i = 0;
    let mut count = 0;
    while polar.len() > 0 {
        i = i % polar.len();
        let curr = polar[i];
        if (curr.y - last_theta).abs() < 2.0*std::f32::EPSILON {
            i = i + 1;
            continue;
        }
        polar.remove(i);
        count += 1;
        match count {
            1 | 2 | 3 | 10 | 20 | 50 | 100 | 199 | 200 | 201 => {
                println!("{}: {:?}", count, to_euclidean(&curr) + base)
            }
            _ => (),
        };
        last_theta = curr.y;
        if count == 200 {
            //break;
        }
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
