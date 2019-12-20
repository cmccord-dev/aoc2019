use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub mod io;
pub use io::*;
pub mod intcode;
pub use intcode::*;

pub fn get_input<T: std::str::FromStr>(name: &str) -> Result<Vec<T>, std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input
        .split(',')
        .filter(|line| line.len() > 0)
        .map(|a| a.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>())
}