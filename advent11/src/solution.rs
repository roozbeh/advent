use crate::intcode;
use std::collections::HashMap;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut computer = intcode::Computer::from(input);
    let mut board: HashMap<(isize, isize), isize> = HashMap::new();
    //                 UP    RIGHT    DOWN     LEFT
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut pos_x: isize = 0;
    let mut pos_y: isize = 0;
    let mut direction = 800;

    while !computer.has_halted() {
        // println!("--------------------------------");
        // println!("NEW CYCLE");
        // println!("position ({}, {})", pos_x, pos_y);
        let tile_color = *board.get(&(pos_x, pos_y)).or(Some(&0)).unwrap();
        // println!("tile_color: {}", tile_color);
        computer.write_input(tile_color);
        if computer.is_waiting() {
            computer.resume();
        } else {
            computer.run();
        }
        let color_of_panel = computer.pop_output().unwrap();
        let rotation = computer.pop_output().unwrap();

        // println!("color_of_panel: {} rotation: {}", color_of_panel, rotation);

        if color_of_panel != tile_color {
            // println!("painted ({}, {})", pos_x, pos_y);
            board.insert((pos_x, pos_y), color_of_panel);
        }
        if rotation == 0 {
            direction -= 1
        } else {
            direction += 1
        }
        let (change_x, change_y) = directions[direction as usize % 4];
        // println!("change_x: {} change_y: {}", change_x, change_y);
        pos_x += change_x;
        pos_y += change_y;
    }
    board.keys().len()
}

pub fn part2(input: &str) -> impl Display {
    let mut computer = intcode::Computer::from(input);
    let mut board: HashMap<(isize, isize), isize> = HashMap::new();
    //                 UP    RIGHT    DOWN     LEFT
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut pos_x: isize = 0;
    let mut pos_y: isize = 5;
    let mut direction = 800;

    board.insert((pos_x, pos_y), 1);

    while !computer.has_halted() {
        // println!("--------------------------------");
        // println!("NEW CYCLE");
        // println!("position ({}, {})", pos_x, pos_y);
        let tile_color = *board.get(&(pos_x, pos_y)).or(Some(&0)).unwrap();
        // println!("tile_color: {}", tile_color);
        computer.write_input(tile_color);
        if computer.is_waiting() {
            computer.resume();
        } else {
            computer.run();
        }
        let color_of_panel = computer.pop_output().unwrap();
        let rotation = computer.pop_output().unwrap();

        // println!("color_of_panel: {} rotation: {}", color_of_panel, rotation);

        if color_of_panel != tile_color {
            // println!("painted ({}, {})", pos_x, pos_y);
            board.insert((pos_x, pos_y), color_of_panel);
        }
        if rotation == 0 {
            direction -= 1
        } else {
            direction += 1
        }
        let (change_x, change_y) = directions[direction as usize % 4];
        // println!("change_x: {} change_y: {}", change_x, change_y);
        pos_x += change_x;
        pos_y += change_y;
    }
    let mut final_board = vec![vec![" "; 50]; 10];
    for ((x, y), val) in board.iter() {
        if *val == 1 {
            final_board[*y as usize][*x as usize] = "#";
        }
    }
    for row in final_board {
        println!("{}", row.join(""));
    }

    board.keys().len()
}
