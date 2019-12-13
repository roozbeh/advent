use crate::intcode;
use std::collections::HashMap;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut board = HashMap::new();

    let mut computer = intcode::Computer::from(input);

    while !computer.has_halted() {
        if computer.is_waiting() {
            computer.resume();
        } else {
            computer.run();
        }
    }
    loop {
        let x_out = computer.pop_output();
        let x;
        match x_out {
            Some(value) => x = value,
            _ => break, // output finished
        }
        let y = computer.pop_output().unwrap();
        let tile_id = computer.pop_output().unwrap();
        board.insert((x, y), tile_id);
    }
    board.values().filter(|&&c| c == 2).count()
}

fn board_size(input: &str) -> (usize, usize) {
    let mut computer = intcode::Computer::from(input);
    let mut max_x = 0;
    let mut max_y = 0;
    while !computer.has_halted() {
        if computer.is_waiting() {
            computer.resume();
        } else {
            computer.run();
        }
    }

    loop {
        let x_out = computer.pop_output();
        let x;
        match x_out {
            Some(value) => x = value,
            _ => break, // output finished
        }
        let y = computer.pop_output().unwrap();
        let _ = computer.pop_output().unwrap();
        if x > max_x {
            max_x = x
        }
        if y > max_y {
            max_y = y
        }
    }
    max_x += 1;
    max_y += 1;
    (max_x as usize, max_y as usize)
}

pub fn part2(input: &str) -> impl Display {
    let (max_x, max_y) = board_size(input);

    let mut computer = intcode::Computer::from(input);
    computer.store_mem(0, 2);

    let mut board = vec![vec![0; max_x]; max_y];
    //let mut board = HashMap::new();
    let mut joystick_position = 0;
    let mut current_ball_position = (0, 0);
    let mut current_paddle_position = (0, 0);
    let mut score = 0;
    while !computer.has_halted() {
        // let mut horizontal_paddle = vec![];
        if computer.is_waiting() {
            println!("input {} score {}", joystick_position, score);
            computer.write_input(joystick_position);
            computer.resume();
        } else {
            computer.run();
        }
        //= (0, 0);
        loop {
            let x_out = computer.pop_output();
            let x;
            match x_out {
                Some(value) => x = value,
                _ => break, // output finished
            }
            let y = computer.pop_output().unwrap();
            let tile_id = computer.pop_output().unwrap();
            if (x, y) == (-1, 0) {
                score = tile_id;
                continue;
            }

            if tile_id == 4 {
                current_ball_position = (x, y);
            }
            if tile_id == 3 {
                current_paddle_position = (x, y);
            }
            // initial value is just placeholder
            if current_ball_position.0 > current_paddle_position.0 {
                joystick_position = 1;
            } else if current_ball_position.0 == current_paddle_position.0 {
                joystick_position = 0;
            } else {
                joystick_position = -1
            }
            board[y as usize][x as usize] = tile_id;
        }
        print_board(&board);
        // let horizontal_paddle = board.iter().filter(|&(_, &v)| v == 3).collect::<Vec<_>>();
        // if horizontal_paddle
        //     .iter()
        //     .filter(|&(k, _)| k.1 == current_ball_position.1)
        //     .count()
        //     == 0
        // {
        //     // the paddle is not under the ball
        //     let rightest_paddle_y = horizontal_paddle.iter().map(|&(k, _)| k.1).max().unwrap();
        //     let leftest_paddle_y = horizontal_paddle.iter().map(|&(k, _)| k.1).min().unwrap();
        //     if current_ball_position.1 > rightest_paddle_y {
        //         joystick_position = 1
        //     } else if current_ball_position.1 < leftest_paddle_y {
        //         joystick_position = -1
        //     }
        // } else {
        //     joystick_position = 0
        // }
    }
    score
}

fn tile_to_string(c: &isize) -> String {
    match c {
        0 => " ".to_owned(),
        1 => "W".to_owned(),
        2 => "#".to_owned(),
        3 => "=".to_owned(),
        4 => "o".to_owned(),
        _ => "".to_owned(),
    }
}

fn print_board(board: &Vec<Vec<isize>>) {
    for row in board {
        println!(
            "{}",
            row.iter()
                .map(tile_to_string)
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
