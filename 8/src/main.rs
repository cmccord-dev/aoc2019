use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input = get_input::<char>("input.test", ' ').unwrap();
    let width = 25;
    let height = 6;
    let _num_layers = input.len() / (width * height);
    let layers = input.chunks(width * height);
    /*let vals = layers.map(|layer| {
        layer.iter().fold((0, 0, 0), |prev, curr| match curr {
            '0' => (prev.0 + 1, prev.1, prev.2),
            '1' => (prev.0, prev.1 + 1, prev.2),
            '2' => (prev.0, prev.1, prev.2 + 1),
            _ => panic!(),
        })
    });
    let mut min = (std::i32::MAX, 0, 0);
    vals.for_each(|val| {
        if min.0 > val.0 {
            min = val
        }
    });
    dbg!(min);
    dbg!(min.1 * min.2);*/
    let base_image = vec!['2'; width * height];
    let image = layers.fold(base_image, |mut prev, curr| {
        (0..width * height).for_each(|i| {
            if prev[i] == '2' {
                prev[i] = curr[i];
            }
        });
        prev
    });
    image.chunks(width).for_each(|layer| {
        println!(
            "{}",
            layer
                .iter()
                .map(|x| {
                    match x {
                        '0' => ' ',
                        _ => '#',
                    }
                })
                .collect::<String>()
        )
    });
}

fn get_input<T: std::str::FromStr>(name: &str, _split: char) -> Result<Vec<T>, std::io::Error> {
    let path = Path::new(name);
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    println!("Parsing input");
    Ok(input
        .split("")
        .filter(|line| line.len() > 0)
        .map(|a| a.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>())
}
