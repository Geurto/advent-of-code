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

        let direction =
            signum(numbers[0].parse::<i8>().unwrap() - numbers[1].parse::<i8>().unwrap());

        let safety_level = check_safety(numbers.clone(), direction, 0);
        //println!(
        //    "Numbers: {:?} Safety level: {}",
        //    numbers.clone(),
        //    safety_level.clone()
        //);

        if safety_level <= 1 {
            num_safe += 1;
        }
    }

    println!("Number of safe sequences: {}", num_safe);
    Ok(())
}

fn check_safety(numbers: Vec<&str>, direction: i8, mut safety_level: u8) -> u8 {
    if safety_level > 1 {
        return safety_level;
    }

    for i in 0..numbers.len() - 1 {
        if !compare_number_safety(
            numbers[i].parse::<i8>().unwrap(),
            numbers[i + 1].parse::<i8>().unwrap(),
            direction,
        ) {
            safety_level += 1;

            if i == 0 {
                //println!("Removing first number!");
                let mut numbers_modified_first = numbers.clone();
                numbers_modified_first.remove(i);
                if check_safety(numbers_modified_first, direction, safety_level) == safety_level {
                    return safety_level;
                }
            }
            //println!("Removing number at index {}", i.clone());
            let mut numbers_modified = numbers.clone();
            numbers_modified.remove(i + 1);
            return check_safety(numbers_modified, direction, safety_level);
        }
    }
    safety_level
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
