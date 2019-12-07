use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // parse the file
    let parsed_orbits: Vec<Vec<&str>> = contents
        .trim()
        .lines()
        .map(|l| l.trim().split(")").collect())
        .collect();

    // store in hashmap
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    for orbit in &parsed_orbits {
        let o = orbits.entry(orbit[0]).or_insert(Vec::new());
        o.push(orbit[1]);
    }

    // sort the input to build the orbitals for each planet sequentially
    let mut ordered_orbits: Vec<(&str, &str)> = Vec::new();
    let mut node = "COM";
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(node);
    loop {
        if let Some(newnode) = queue.pop_front() {
            node = newnode;
        } else {
            break;
        }

        if let Some((key, values)) = orbits.get_key_value(&node) {
            for value in values {
                ordered_orbits.push((key, value));
                queue.push_back(value);
            }
        } else {
            // leaf
            continue;
        }
    }

    // build a map with the # of orbitals and parent for each planet
    let mut orbit_checksum: HashMap<&str, (&str, i32)> = HashMap::new();
    orbit_checksum.insert("COM", ("", 0));
    for orbit in &ordered_orbits {
        let mut orbit_entry: (&str, i32) = (orbit.0, 0);
        {
            if let Some(x) = orbit_checksum.get(orbit.0) {
                orbit_entry = (orbit.0, x.1);
            }
        }
        let orbiter_entry = orbit_checksum.entry(orbit.1).or_insert((orbit.0, 0));
        *orbiter_entry = (orbit_entry.0, orbit_entry.1 + 1);
    }

    // part 1 - sum the number of orbitals for each planet
    let mut total = 0;
    for (_parent, value) in orbit_checksum.values() {
        total += value;
    }

    println!("total orbitals {:?}", total);

    // part 2 - walk up the inverted index and diff the paths to the root
    let leaves = vec!["YOU", "SAN"];
    let mut paths: Vec<HashSet<&str>> = Vec::new();

    for leaf in leaves {
        paths.push(HashSet::new());
        node = leaf;
        loop {
            if let Some((parent, _)) = orbit_checksum.get(&node) {
                if *parent != "" {
                    paths.last_mut().unwrap().insert(parent);
                } else {
                    break;
                }
                node = parent;
            }
        }
    }

    let distance: HashSet<_> = paths[0].symmetric_difference(&paths[1]).collect();
    println!("from YOU to SAN {:?}", distance.len());

    Ok(())
}
