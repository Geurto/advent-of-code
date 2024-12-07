use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("Read line {:?}", line);
        let pairs = extract_multiplication_values(&line);

        println!("Extracted values {:?}", pairs.clone());

        let sum = multiply_and_sum(&pairs);

        println!("Computed sum {}", sum);
    }

    Ok(())
}

fn extract_multiplication_values(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\s*\((\d+)\s*,\s*(\d+)\)").unwrap();

    re.captures_iter(input)
        .filter_map(|cap| {
            let x = cap[1].parse::<i32>().ok()?;
            let y = cap[2].parse::<i32>().ok()?;

            Some((x, y))
        })
        .collect()
}

fn multiply_and_sum(pairs: &[(i32, i32)]) -> i32 {
    pairs.iter().map(|(x, y)| x * y).sum()
}
