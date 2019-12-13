use itertools::Itertools;
use num::integer::Integer;
use regex::Regex;
use std::fmt::Display;

fn sign(a: isize) -> isize {
    if a > 0 {
        1
    } else if a < 0 {
        -1
    } else {
        0
    }
}

fn sum(v: &Vec<isize>) -> isize {
    v.iter().map(|x| x.abs()).sum::<isize>()
}

fn parse(line: &str) -> (Vec<isize>, Vec<isize>) {
    let re: Regex = Regex::new(r"^<x=(\-?\d+), y=(\-?\d+), z=(\-?\d+)>$").unwrap();
    let cap = re.captures(line).unwrap();
    (
        vec![
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ],
        vec![0, 0, 0],
    )
}

pub fn part1(input: &str) -> impl Display {
    let mut moons: Vec<(Vec<isize>, Vec<isize>)> = Vec::new();
    for line in input.lines() {
        moons.push(parse(line.trim()));
    }
    let moons_number = moons.len();
    let mut total_energy = 0;

    for step in 0..2773 {
        // gravity phase
        for (moon1_id, moon2_id) in (0..moons_number).tuple_combinations() {
            let mut moon1 = moons[moon1_id].clone();
            let mut moon2 = moons[moon2_id].clone();

            for i in 0..3 {
                let delta = sign(moon1.0[i] - moon2.0[i]);
                moon1.1[i] += -delta;
                moon2.1[i] += delta;
            }
            moons[moon1_id] = moon1;
            moons[moon2_id] = moon2;
        }

        for moon in moons.iter_mut() {
            for i in 0..3 {
                moon.0[i] += moon.1[i];
            }
        }

        if step == 999 {
            for moon in &moons {
                total_energy += sum(&moon.0) * sum(&moon.1);
            }
        }
    }
    total_energy
}

pub fn part2(input: &str) -> usize {
    let mut moons: Vec<(Vec<isize>, Vec<isize>)> = Vec::new();
    for line in input.lines() {
        moons.push(parse(line.trim()));
    }

    let mut axes_cycles = vec![];
    for axes in 0..3 {
        axes_cycles.push(find_cycle_in_axis(&mut moons.clone(), axes));
    }
    axes_cycles[2].lcm(&axes_cycles[0].lcm(&axes_cycles[1]))
}

fn find_cycle_in_axis(moons: &mut Vec<(Vec<isize>, Vec<isize>)>, axes: usize) -> usize {
    let initial = get_axes(moons, axes);
    let moons_number = moons.len();

    let mut step = 0;
    loop {
        for (moon1_id, moon2_id) in (0..moons_number).tuple_combinations() {
            let mut moon1 = moons[moon1_id].clone();
            let mut moon2 = moons[moon2_id].clone();

            // gravity phase
            let delta = sign(moon1.0[axes] - moon2.0[axes]);
            moon1.1[axes] += -delta;
            moon2.1[axes] += delta;

            moons[moon1_id] = moon1;
            moons[moon2_id] = moon2;
        }
        // velocity phase
        for moon in moons.iter_mut() {
            moon.0[axes] += moon.1[axes];
        }
        step += 1;

        if get_axes(&moons, axes) == initial {
            return step;
        }
    }
}

fn get_axes(moons: &Vec<(Vec<isize>, Vec<isize>)>, axes: usize) -> Vec<(isize, isize)> {
    moons
        .iter()
        .map(|v| (v.0[axes], v.1[axes]))
        .collect::<Vec<(isize, isize)>>()
}

// fn print_moons(moons: &Vec<(Vec<isize>, Vec<isize>)>) {
//     for (i, moon) in moons.iter().enumerate() {
//         println!(
//             "moon {}{:?} {}{:?} {}{:?}",
//             i,
//             (moon.0[0], moon.1[0]),
//             i,
//             (moon.0[1], moon.1[1]),
//             i,
//             (moon.0[2], moon.1[2])
//         );
//     }
// }
