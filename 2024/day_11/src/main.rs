use std::collections::HashMap;
use std::fs;

use env_logger::Builder;
use log::info;

fn main() {
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    let data = fs::read_to_string("data/dummy").expect("Unable to read file");
    let mut stones: Vec<usize> = data
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut stones_map: HashMap<usize, Vec<usize>> = HashMap::new();
    stones_map.insert(0, vec![1]);

    for i in 0..25 {
        info!("#{}: {} stones", i, stones.len());
        blink(&mut stones, &mut stones_map);
    }
}

fn blink(stones: &mut Vec<usize>, map: &mut HashMap<usize, Vec<usize>>) {
    let stones_taken = std::mem::take(stones);
    for (i, s) in stones_taken.into_iter().enumerate() {
        if let Some(new_stones) = map.get(&s) {
            stones[i] = new_stones[0];
            if new_stones.len() == 2 {
                stones.push(new_stones[1]);
            }
        }
    }
}
