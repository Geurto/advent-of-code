use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);
    let mut sum_solvable_results = 0;

    for line in reader.lines().map_while(Result::ok) {
        let split: Vec<&str> = line.split(": ").collect();
        let result = split[0].parse::<usize>().unwrap();
        let factors: Vec<usize> = split[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        if subdivide((result, &factors)) {
            sum_solvable_results += result;
        }
    }

    println!("Sum of solvable results: {}", sum_solvable_results);

    Ok(())
}

fn subdivide(equation: (usize, &[usize])) -> bool {
    let (result, factors) = equation;

    if factors.is_empty() {
        return false;
    } else if factors.len() == 1 {
        return result == factors[0];
    }

    let k = factors[factors.len() - 1];

    if result == k {
        return true;
    }

    if result < k {
        return false;
    }

    // if k is a factor, then continue to subdivide
    if result % k == 0 {
        return subdivide((result / k, &factors[..factors.len() - 1]));
    }

    subdivide((result - k, &factors[..factors.len() - 1]))
}
