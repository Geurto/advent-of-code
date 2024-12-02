use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0].parse().unwrap());
            right_list.push(numbers[1].parse().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();

    let mut sum_of_differences = 0;
    let mut similarity_sum = 0;

    for i in 0..left_list.len() {
        let left_value = left_list[i];
        sum_of_differences += (left_list[i] - right_list[i]).abs();
        similarity_sum +=
            left_value * right_list.iter().filter(|&x| x == &left_value).count() as i32;
    }

    println!("Sum of differences: {}", sum_of_differences);
    println!("Similarity sum: {}", similarity_sum);

    Ok(())
}
