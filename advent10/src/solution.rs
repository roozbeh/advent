use num::range_step;
use num::Integer;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let grid = input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut view_grid: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, loc) in row.iter().enumerate() {
            if *loc == '.' {
                continue;
            } else {
                view_grid[i][j] = count_from(&grid, i, j);
            }
        }
    }

    let mut highest: usize = 0;
    let mut coords: (usize, usize) = (0, 0);
    for (i, row) in view_grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value > highest {
                coords = (i, j);
                highest = *value;
            }
        }
    }

    println!("{:?} {:?}", coords, highest);
    0
}

fn count_from(grid: &Vec<Vec<char>>, from_i: usize, from_j: usize) -> usize {
    let mut total_seen: usize = 0;
    for (to_i, row) in grid.iter().enumerate() {
        for (to_j, loc) in row.iter().enumerate() {
            if *loc == '.' {
                continue;
            }
            if from_i == to_i && from_j == to_j {
                continue;
            }

            if can_see(
                grid,
                from_i as isize,
                from_j as isize,
                to_i as isize,
                to_j as isize,
            ) {
                total_seen += 1
            }
        }
    }
    total_seen
}

fn can_see(grid: &Vec<Vec<char>>, from_i: isize, from_j: isize, to_i: isize, to_j: isize) -> bool {
    if from_i == to_i {
        // same row
        let range_j = range_step(from_j, to_j, if from_j > to_j { -1 } else { 1 });
        for candidate_j in range_j {
            if candidate_j == from_j {
                continue;
            }
            if grid[from_i as usize][candidate_j as usize] != '.' {
                return false;
            }
        }
        return true;
    }

    if from_j == to_j {
        //same column
        let range_i = range_step(from_i, to_i, if from_i > to_i { -1 } else { 1 });
        for candidate_i in range_i {
            if candidate_i == from_i {
                continue;
            }
            if grid[candidate_i as usize][from_j as usize] != '.' {
                return false;
            }
        }
        return true;
    }

    // this is basically calculating the minimum steps and then going through
    // the range between point A and B.
    let i_diff = to_i - from_i;
    let j_diff = to_j - from_j;

    let divisor = i_diff.gcd(&j_diff);

    let i_step = i_diff / divisor;
    let j_step = j_diff / divisor;

    let range_i = range_step(from_i, to_i, i_step).collect::<Vec<isize>>();
    let range_j = range_step(from_j, to_j, j_step).collect::<Vec<isize>>();

    for (candidate_i, candidate_j) in range_i[1..].iter().zip(range_j[1..].iter()) {
        if grid[*candidate_i as usize][*candidate_j as usize] != '.' {
            return false;
        }
    }
    true
}

pub fn part2(input: &str) -> impl Display {
    // repeat
    // get all visible from (19, 22)
    // sort by angle
    //     angle    is let i_diff = to_i - from_i;
    //                 let j_diff = to_j - from_j;
    //                 let pendenza = i_diff / j_diff;

    // remove in order, count to 200, return it
    0
}
