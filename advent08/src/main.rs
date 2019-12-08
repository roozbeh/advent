use std::env;
use std::fs::File;
use std::io::prelude::*;
mod solution;
use itertools::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let width = args[2].parse::<usize>().unwrap();
    let height = args[3].parse::<usize>().unwrap();

    println!(
        "checksum {}",
        solution::part1(&contents, width, height).unwrap()
    );

    // part 2
    let final_image = solution::part2(&contents, width, height).unwrap();
    for row in final_image.chunks(width) {
        println!("{}", row.iter().join(""));
    }

    Ok(())
}
