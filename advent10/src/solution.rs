use num::range_step;
use num::Integer;

pub fn part1(input: &str) -> (usize, (usize, usize)) {
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
                let asteroids_seen = count_from(&grid, i, j);
                view_grid[i][j] = asteroids_seen.len();
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
    (highest, coords)
}

fn count_from(grid: &Vec<Vec<char>>, from_i: usize, from_j: usize) -> Vec<(usize, usize)> {
    let mut total_seen: Vec<(usize, usize)> = Vec::new();
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
                total_seen.push((to_i, to_j));
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

pub fn part2(input: &str) -> impl std::fmt::Debug {
    let mut grid = input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (_, from): (usize, (usize, usize)) = part1(input);
    let mut removed: Vec<(usize, usize)> = Vec::new();
    loop {
        let mut visible_ones = count_from(&grid, from.0, from.1);
        if visible_ones.len() == 0 {
            break;
        }
        visible_ones.sort_by(|a, b| {
            let slope_a = slope(from, *a);
            let slope_b = slope(from, *b);
            slope_a.partial_cmp(&slope_b).unwrap()
        });
        for o in visible_ones {
            removed.push(o);
            grid[o.0][o.1] = '.';
        }
    }
    let number_bet = removed[199];
    number_bet.1 * 100 + number_bet.0
}

fn slope(from: (usize, usize), to: (usize, usize)) -> f64 {
    let i_diff = to.0 as isize - from.0 as isize;
    let j_diff = to.1 as isize - from.1 as isize;

    let angle = (j_diff as f64).atan2(-i_diff as f64).to_degrees();
    if angle < 0.0 {
        return 360.0 + angle;
    }
    angle
}
