use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let origintcodes: Vec<u32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("In: {:?}", origintcodes);

    for noun in 1..99 {
        for verb in 1..99 {
            let mut curr_position = 0;
            let mut intcodes = origintcodes.clone();
            intcodes[1] = noun;
            intcodes[2] = verb;
            loop {
                match intcodes[curr_position] {
                    1 => {
                        let a = intcodes[curr_position + 1] as usize;
                        let b = intcodes[curr_position + 2] as usize;
                        let store_pos = intcodes[curr_position + 3] as usize;
                        intcodes[store_pos] = intcodes[a] + intcodes[b];
                        curr_position += 4
                    }
                    2 => {
                        let a = intcodes[curr_position + 1] as usize;
                        let b = intcodes[curr_position + 2] as usize;
                        let store_pos = intcodes[curr_position + 3] as usize;
                        intcodes[store_pos] = intcodes[a] * intcodes[b];
                        curr_position += 4
                    }
                    99 => break,
                    v => {
                        println!("Error, unexpected: {:?}", v);
                        break;
                    }
                }
            }
            if intcodes[0] == 19690720 {
                println!("Out: {:?}", intcodes[0]);
                println!("noun: {:?}, verb: {:?}", noun, verb);
                println!("out: {:?}", noun * 100 + verb);
            }
        }
    }
    Ok(())
}
