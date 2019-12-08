use itertools::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_layer(data: &[u8], layer_num: usize, layer_size: usize) -> &[u8] {
    let base = layer_num * layer_size;
    &data[base..base + layer_size]
}

fn part1(contents: &String, width: usize, height: usize) {
    //let data: Vec<u8> = contents.trim().bytes().map(|digit| digit - 48).collect();
    //let layers_number = data.len() / (width * height);

    // part 1
    // let mut min_zero_idx: usize = 0;
    // let mut min_zero_count: usize = usize::max_value();
    // for layer_idx in 0..layers_number {
    //     let layer = get_layer(&data, layer_idx, width * height);
    //     let zero_count = layer.iter().filter(|&&x| x == 0).count();
    //     if zero_count < min_zero_count {
    //         min_zero_count = zero_count;
    //         min_zero_idx = layer_idx;
    //     }
    // }
    // let smallest = get_layer(&data, min_zero_idx, width * height);
    let smallest: Vec<u8> = contents
        .trim()
        .bytes()
        .map(|digit| digit - 48)
        .chunks(width * height)
        .into_iter()
        .map(|x| x.collect::<Vec<u8>>())
        .min_by_key(|x| x.iter().filter(|&&c| c == 0).count())
        .unwrap();
    println!(
        "checksum {:?}",
        smallest.iter().filter(|&&x| x == 1).count() * smallest.iter().filter(|&&x| x == 2).count()
    );
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let width = args[2].parse::<usize>().unwrap();
    let height = args[3].parse::<usize>().unwrap();

    part1(&contents, width, height);

    // part 2
    let data: Vec<u8> = contents.trim().bytes().map(|digit| digit - 48).collect();
    let layers_number = data.len() / (width * height);

    let mut final_image = vec![""; width * height];
    for layer_idx in 0..layers_number {
        let current_layer = get_layer(&data, layer_idx, width * height);
        for pixel_idx in 0..width * height {
            if final_image[pixel_idx] == "" {
                let p = current_layer[pixel_idx];
                final_image[pixel_idx] = if p == 0 {
                    " "
                } else if p == 1 {
                    "*"
                } else {
                    ""
                };
            }
        }
    }

    for row in final_image.chunks(width) {
        println!("{}", row.iter().join(""));
    }

    Ok(())
}
