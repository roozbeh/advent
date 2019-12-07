use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let origintcodes: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("In: {:?}", origintcodes);

    let mut curr_position = 0;
    let mut intcodes = origintcodes.clone();
    let stdin = io::stdin();
    loop {
        let opcode = intcodes[curr_position] % 100;
        let mode = parse_mode(intcodes[curr_position] / 100);
        //println!("opcode: {:?} mode: {:?}", opcode, mode);
        match opcode {
            1 => {
                let a = intcodes[curr_position + 1];
                let b = intcodes[curr_position + 2];
                let store_pos = intcodes[curr_position + 3] as usize;
                let first_param = load(mode[0], &intcodes, a);
                let second_param = load(mode[1], &intcodes, b);
                intcodes[store_pos] = first_param + second_param;
                curr_position += 4
            }
            2 => {
                let a = intcodes[curr_position + 1];
                let b = intcodes[curr_position + 2];
                let store_pos = intcodes[curr_position + 3] as usize;
                let first_param = load(mode[0], &intcodes, a);
                let second_param = load(mode[1], &intcodes, b);
                intcodes[store_pos] = first_param * second_param;
                curr_position += 4
            }
            3 => {
                println!("INSERT INPUT: ");
                let store_pos = intcodes[curr_position + 1] as usize;
                let mut input = String::new();
                stdin.lock().read_line(&mut input).unwrap();
                intcodes[store_pos] = input.trim().parse::<i32>().unwrap();
                curr_position += 2
            }
            4 => {
                let a = intcodes[curr_position + 1];
                let output = load(mode[0], &intcodes, a);
                println!("OUT: {:?}", output);
                curr_position += 2
            }
            5 => {
                let a = intcodes[curr_position + 1];
                let first_param = load(mode[0], &intcodes, a);
                if first_param != 0 {
                    let b = intcodes[curr_position + 2];
                    let second_param = load(mode[1], &intcodes, b);
                    curr_position = second_param as usize;
                } else {
                    curr_position += 3
                }
            }
            6 => {
                let a = intcodes[curr_position + 1];
                let first_param = load(mode[0], &intcodes, a);
                if first_param == 0 {
                    let b = intcodes[curr_position + 2];
                    let second_param = load(mode[1], &intcodes, b);
                    curr_position = second_param as usize;
                } else {
                    curr_position += 3
                }
            }
            7 => {
                let a = intcodes[curr_position + 1];
                let b = intcodes[curr_position + 2];
                let store_pos = intcodes[curr_position + 3] as usize;
                let first_param = load(mode[0], &intcodes, a);
                let second_param = load(mode[1], &intcodes, b);
                if first_param < second_param {
                    intcodes[store_pos] = 1
                } else {
                    intcodes[store_pos] = 0
                }
                curr_position += 4
            }
            8 => {
                let a = intcodes[curr_position + 1];
                let b = intcodes[curr_position + 2];
                let store_pos = intcodes[curr_position + 3] as usize;
                let first_param = load(mode[0], &intcodes, a);
                let second_param = load(mode[1], &intcodes, b);
                if first_param == second_param {
                    intcodes[store_pos] = 1
                } else {
                    intcodes[store_pos] = 0
                }
                curr_position += 4
            }

            99 => break,
            v => {
                println!("Error, unexpected opcode: {:?}", v);
                break;
            }
        }
    }
    Ok(())
}

fn load(mode: i32, intcodes: &Vec<i32>, cp: i32) -> i32 {
    match mode {
        0 => intcodes[cp as usize],
        1 => cp,
        _ => {
            println!("Error, unexpected mode: {:?}", mode);
            0
        }
    }
}

fn parse_mode(mode: i32) -> Vec<i32> {
    let mut v = Vec::new();
    v.push(mode % 10);
    v.push(mode / 10 % 10);
    v.push(mode / 100 % 10);
    v
}
