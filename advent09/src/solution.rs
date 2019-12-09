use crate::intcode;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut computer = intcode::Computer::from(input);
    while !computer.has_halted() {
        if computer.is_waiting() {
            println!("waiting");
            computer.write_input(1);
            computer.resume();
        } else {
            computer.run();
        }
    }
    let mut output: Vec<i128> = Vec::new();
    loop {
        match computer.pop_output() {
            Some(value) => {
                output.push(value);
            }
            None => break,
        }
    }
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part2(input: &str) -> impl Display {
    let mut computer = intcode::Computer::from(input);
    while !computer.has_halted() {
        if computer.is_waiting() {
            println!("waiting");
            computer.write_input(2);
            computer.resume();
        } else {
            computer.run();
        }
    }
    let mut output: Vec<i128> = Vec::new();
    loop {
        match computer.pop_output() {
            Some(value) => {
                output.push(value);
            }
            None => break,
        }
    }
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
