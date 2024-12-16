use std::collections::HashMap;
use std::fs;

use env_logger::Builder;
use log::{debug, info};

fn main() {
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    let data = fs::read_to_string("data/input").expect("Unable to read file");
    let stones: Vec<usize> = data
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // index, (count, next)
    let mut result_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut stones_map: HashMap<usize, usize> = HashMap::new();

    // initialize stones
    for s in stones.iter() {
        let count = stones_map.entry(*s).or_insert(0);
        *count += 1;
    }

    // initialize trivial results
    result_map.insert(0, vec![1]);
    result_map.insert(1, vec![2024]);

    for i in 0..75 {
        stones_map = blink(&mut stones_map, &mut result_map);
        let num_stones = stones_map.values().sum::<usize>();
        info!("# {}.    {} stones", i + 1, num_stones);
    }
    debug!("Hashmap size: {}", result_map.len());
    debug!(
        "Result hashmap: {:?}",
        result_map.iter().collect::<Vec<_>>()
    );
}

fn blink(
    previous_stones: &mut HashMap<usize, usize>,
    map: &mut HashMap<usize, Vec<usize>>,
) -> HashMap<usize, usize> {
    let mut new_counts: HashMap<usize, usize> = HashMap::new();

    // check each unique number on previous stones
    for (s, stone_count) in previous_stones.iter_mut() {
        let next_stones = map.entry(*s).or_insert_with(|| {
            let digits = (*s as f64).log10() as usize + 1;
            if digits % 2 == 0 {
                let divisor = 10usize.pow(digits as u32 / 2);
                let (left, right) = (*s / divisor, *s % divisor);
                vec![left, right]
            } else {
                vec![*s * 2024]
            }
        });

        let count = new_counts.entry(next_stones[0]).or_insert(0);
        *count += *stone_count;

        if next_stones.len() == 2 {
            let count = new_counts.entry(next_stones[1]).or_insert(0);
            *count += *stone_count;
        }
    }

    new_counts
}
