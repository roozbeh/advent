// use itertools::*;

fn get_layer(data: &[u8], layer_num: usize, layer_size: usize) -> &[u8] {
    let base = layer_num * layer_size;
    &data[base..base + layer_size]
}

pub fn part1(input: &str, width: usize, height: usize) -> std::io::Result<usize> {
    let data: Vec<u8> = input.trim().bytes().map(|digit| digit - 48).collect();
    let layers_number = data.len() / (width * height);

    let mut min_zero_idx: usize = 0;
    let mut min_zero_count: usize = usize::max_value();
    for layer_idx in 0..layers_number {
        let layer = get_layer(&data, layer_idx, width * height);
        let zero_count = layer.iter().filter(|&&x| x == 0).count();
        if zero_count < min_zero_count {
            min_zero_count = zero_count;
            min_zero_idx = layer_idx;
        }
    }
    let smallest = get_layer(&data, min_zero_idx, width * height);

    // This looks leet and it's all basically a single line but...
    // p1_me              time:   [117.04 us 119.04 us 121.15 us]
    //                    change: [+2422.8% +2505.4% +2586.4%] (p = 0.00 < 0.05)
    // it's way worse than the optimal above.
    // let smallest: Vec<u8> = input
    //     .trim()
    //     .bytes()
    //     .map(|digit| digit - 48)
    //     .chunks(width * height)
    //     .into_iter()
    //     .map(|x| x.collect::<Vec<u8>>())
    //     .min_by_key(|x| x.iter().filter(|&&c| c == 0).count())
    //     .unwrap();
    let result =
        smallest.iter().filter(|&&x| x == 1).count() * smallest.iter().filter(|&&x| x == 2).count();
    Ok(result)
}

pub fn part2(input: &str, width: usize, height: usize) -> std::io::Result<Vec<&str>> {
    let data: Vec<u8> = input.trim().bytes().map(|digit| digit - 48).collect();
    let layers_number = data.len() / (width * height);

    let mut final_image = vec![""; width * height];
    for layer_idx in 0..layers_number {
        let current_layer = get_layer(&data, layer_idx, width * height);
        for pixel_idx in 0..width * height {
            if final_image[pixel_idx] == "" {
                let p = current_layer[pixel_idx];
                final_image[pixel_idx] = match p {
                    0 => " ",
                    1 => "*",
                    _ => "",
                }
            }
        }
    }

    Ok(final_image)
}
