use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let mut num_safe: i16 = 0;
    let mut num_safe_with_dampener: i16 = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if check_safety(numbers.clone()) {
            num_safe += 1;
        }

        if check_safety_with_dampener(numbers.clone()) {
            num_safe_with_dampener += 1;
        }
    }

    println!("Number of safe sequences: {}", num_safe);
    println!(
        "Number of safe sequences with dampener: {}",
        num_safe_with_dampener
    );
    Ok(())
}

fn check_safety(numbers: Vec<i32>) -> bool {
    let safe_increasing = numbers.windows(2).all(is_safe_increasing);
    let safe_decreasing = numbers.windows(2).all(is_safe_decreasing);

    safe_increasing || safe_decreasing
}

fn check_safety_with_dampener(numbers: Vec<i32>) -> bool {
    let mut safe_increasing: bool = false;
    let mut safe_decreasing: bool = false;

    for i in 0..numbers.len() {
        let numbers_modified = numbers
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| i != *idx)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>();

        safe_increasing = numbers_modified.windows(2).all(is_safe_increasing);
        safe_decreasing = numbers_modified.windows(2).all(is_safe_decreasing);

        if safe_increasing || safe_decreasing {
            break;
        }
    }
    safe_increasing || safe_decreasing
}

fn is_safe_increasing(pair: &[i32]) -> bool {
    pair[1] > pair[0] && (pair[1] - pair[0]) > 0 && (pair[1] - pair[0]) < 4
}

fn is_safe_decreasing(pair: &[i32]) -> bool {
    pair[0] > pair[1] && (pair[0] - pair[1]) > 0 && (pair[0] - pair[1]) < 4
}
