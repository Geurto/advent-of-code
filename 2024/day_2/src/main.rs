use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let mut num_safe: i16 = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 0 {
            continue;
        }

        let mut index_unsafe: i8 = -1;
        let direction =
            signum(numbers[0].parse::<i8>().unwrap() - numbers[1].parse::<i8>().unwrap());

        index_unsafe = check_safety(numbers.clone(), direction);

        if index_unsafe == -1 {
            num_safe += 1;
        }
    }

    println!("Number of safe sequences: {}", num_safe);
    Ok(())
}

fn check_safety(numbers: Vec<&str>, direction: i8) -> i8 {
    for i in 0..numbers.len() - 1 {
        if !compare_number_safety(
            numbers[i].parse::<i8>().unwrap(),
            numbers[i + 1].parse::<i8>().unwrap(),
            direction,
        ) {
            return i as i8;
        }
    }
    -1
}

fn compare_number_safety(a: i8, b: i8, d: i8) -> bool {
    let diff = (a - b).abs();
    let sign = signum(a - b);

    (diff >= 1) && (diff <= 3) && (sign == d)
}

fn signum(a: i8) -> i8 {
    if a < 0 {
        -1
    } else if a == 0 {
        0
    } else {
        1
    }
}
