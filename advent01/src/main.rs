use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut weights: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            continue;
        }
        let weight: u32 = line.parse().unwrap();
        weights.push(weight);
    }

    println!("{}", weights.into_iter().map(|x| (x / 3) - 2).sum::<u32>(),);

    Ok(())
}
