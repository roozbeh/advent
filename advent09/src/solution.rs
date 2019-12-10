use std::fmt::Display;

include!("intcode.rs");

pub fn part1(input: &str) -> impl Display {
    let mut computer = Computer::from(input);
    computer.write_input(1);
    computer.run();

    computer
        .get_output()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part2(input: &str) -> impl Display {
    let mut computer = Computer::from(input);
    computer.write_input(2);
    computer.run();
    computer
        .get_output()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
