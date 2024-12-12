use std::fs::File;
use std::io::{BufRead, BufReader};

use env_logger::Builder;
use log::debug;

fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Info).init();

    let file = File::open("data/input")?;
    let reader = BufReader::new(file);
    let mut sum_solvable_results = 0;
    let mut sum_solvable_results_with_concatenation = 0;

    for line in reader.lines().map_while(Result::ok) {
        let split: Vec<&str> = line.split(": ").collect();
        let result = split[0].parse::<usize>().unwrap();
        let mut factors: Vec<usize> = split[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        if subdivide((result, &mut factors)) {
            sum_solvable_results += result;
        }

        if subdivide_with_concatenation((result, &mut factors)) {
            debug!("Solved with concatenation: {}: {:?}", result, factors);
            sum_solvable_results_with_concatenation += result;
        }
    }

    println!("Sum of solvable results: {}", sum_solvable_results);
    println!(
        "Sum of solvable results with concatenation: {}",
        sum_solvable_results_with_concatenation
    );

    Ok(())
}

fn subdivide(equation: (usize, &[usize])) -> bool {
    let (result, factors) = equation;
    if factors.len() == 1 {
        return result == factors[0];
    }

    let k = factors[factors.len() - 1];
    let remaining = &factors[..factors.len() - 1];

    // if k is a factor, then continue to subdivide
    if result % k == 0 {
        if subdivide((result / k, remaining)) {
            return true;
        }
        if k > result {
            return false;
        }
        // see if we can solve by subtracting at this step instead
        return subdivide((result - k, remaining));
    }

    // else, subtract k and go one deeper
    if result >= k {
        return subdivide((result - k, remaining));
    }

    false
}

fn subdivide_with_concatenation(equation: (usize, &mut [usize])) -> bool {
    let (result, factors) = equation;
    if factors.len() == 1 {
        return result == factors[0];
    }

    let k = factors[factors.len() - 1];
    let num_remaining = factors.len() - 1;
    let remaining = &mut factors[..num_remaining];

    if can_divide(result, k) && subdivide_with_concatenation((result / k, remaining)) {
        return true;
    }
    if can_subtract(result, k) && subdivide_with_concatenation((result - k, remaining)) {
        return true;
    }
    if can_decatenate(result, k) {
        let decatenated = decatenate(result, k);
        if subdivide_with_concatenation((decatenated, remaining)) {
            return true;
        }
    }

    false
}

fn decatenate(number: usize, digits: usize) -> usize {
    let divisor = 10_usize.pow(digits.to_string().len() as u32);
    debug!(
        "Decatenating {} with {} into {}",
        number,
        digits,
        number / divisor
    );
    number / divisor
}

fn can_divide(number: usize, divisor: usize) -> bool {
    number % divisor == 0
}

fn can_subtract(number: usize, subtrahend: usize) -> bool {
    number >= subtrahend
}

fn can_decatenate(number: usize, digits: usize) -> bool {
    let divisor = 10_usize.pow(digits.to_string().len() as u32);
    number % divisor == digits
}
