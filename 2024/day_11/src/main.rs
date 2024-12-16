use std::collections::HashMap;
use std::fs;

use env_logger::Builder;
use log::{debug, info};

fn main() {
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    let data = fs::read_to_string("data/dummy").expect("Unable to read file");
    let mut stones: Vec<usize> = data
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // index, (count, next)
    let mut hashmap: HashMap<usize, (usize, Vec<usize>)> = HashMap::new();
    hashmap.insert(0, (0, vec![1]));
    hashmap.insert(1, (0, vec![2024]));

    for i in 0..25 {
        if i < 10 {
            debug!("Stones: {:?}", stones);
        }
        blink(&mut stones, &mut hashmap);
        info!("# {}.    {} stones", i + 1, stones.len());
    }
    debug!("Hashmap size: {}", hashmap.len());
    debug!(
        "First 10 elements of hashmap: {:?}",
        hashmap.iter().collect::<Vec<_>>()
    );
}

fn blink(stones: &mut Vec<usize>, map: &mut HashMap<usize, (usize, Vec<usize>)>) {
    let mut new_stones = Vec::new();

    for s in stones.iter_mut() {
        let (count, next_stones) = map.entry(*s).or_insert_with(|| {
            let digits = (*s as f64).log10() as usize + 1;
            if digits % 2 == 0 {
                let divisor = 10usize.pow(digits as u32 / 2);
                let (left, right) = (*s / divisor, *s % divisor);
                (0, vec![left, right])
            } else {
                (0, vec![*s * 2024])
            }
        });

        *count += 1;
        let next_stones = next_stones.clone();

        *s = next_stones[0];
        if next_stones.len() > 1 {
            new_stones.push(next_stones[1]);
        }
    }
    stones.append(&mut new_stones);
}
