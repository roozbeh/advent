use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut wires: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut wires_locations: Vec<HashMap<(i32, i32), i32>> = Vec::new();

    let start = (0i32, 0i32);
    for wire in reader.lines() {
        let wire = wire?;
        if wire == "" {
            continue;
        }

        wires.push(HashSet::new());
        wires_locations.push(HashMap::new());

        let mut position = start.clone();
        let wire_movements: Vec<&str> = wire.trim().split(",").collect();

        let mut total_steps = 0i32;
        for mmove in wire_movements {
            let (direction, movement) = mmove.split_at(1);
            let steps = movement.parse::<i32>().unwrap();
            match direction {
                "R" => {
                    for i in 0..steps {
                        position.0 += 1;
                        wires.last_mut().unwrap().insert(position.clone());
                        if !wires_locations.last().unwrap().contains_key(&position) {
                            wires_locations
                                .last_mut()
                                .unwrap()
                                .insert(position, total_steps + i + 1);
                        }
                    }
                }
                "L" => {
                    for i in 0..steps {
                        position.0 -= 1;
                        wires.last_mut().unwrap().insert(position.clone());
                        if !wires_locations.last().unwrap().contains_key(&position) {
                            wires_locations
                                .last_mut()
                                .unwrap()
                                .insert(position, total_steps + i + 1);
                        }
                    }
                }
                "U" => {
                    for i in 0..steps {
                        position.1 += 1;
                        wires.last_mut().unwrap().insert(position.clone());
                        if !wires_locations.last().unwrap().contains_key(&position) {
                            wires_locations
                                .last_mut()
                                .unwrap()
                                .insert(position, total_steps + i + 1);
                        }
                    }
                }
                "D" => {
                    for i in 0..steps {
                        position.1 -= 1;
                        wires.last_mut().unwrap().insert(position.clone());
                        if !wires_locations.last().unwrap().contains_key(&position) {
                            wires_locations
                                .last_mut()
                                .unwrap()
                                .insert(position, total_steps + i + 1);
                        }
                    }
                }
                _ => println!("Unknown direction: {:?}", direction),
            }
            total_steps += steps;
        }
    }

    let crosses = wires[0].intersection(&wires[1]);
    println!("crosses {:?}", crosses);
    let mut candidate = 10000000;
    let mut distance_candidate = 10000000;
    for cross in crosses {
        let mut distance = 0i32;
        for wire_location in &wires_locations {
            if let Some(entry) = wire_location.get(cross) {
                distance += entry;
            }
        }

        let current_candidate = cross.0.abs() + cross.1.abs();
        if current_candidate < candidate {
            candidate = current_candidate;
        }
        if distance < distance_candidate {
            distance_candidate = distance;
        }
    }
    println!("{:?}", candidate);
    println!("{:?}", distance_candidate);
    Ok(())
}
