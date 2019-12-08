use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let weights: Vec<isize> = contents
        .trim()
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    // part 1
    println!("{}", weights.iter().map(|x| (x / 3) - 2).sum::<isize>());

    // part 2
    println!(
        "{}",
        weights
            .iter()
            .map(|&x| fuel_for_module(0, x))
            .sum::<isize>()
    );

    Ok(())
}

fn fuel_for_module(total: isize, mass: isize) -> isize {
    let fuel: isize = mass / 3 - 2;
    if fuel <= 0 {
        return total;
    }
    return fuel_for_module(total + fuel, fuel);
}
