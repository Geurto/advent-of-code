use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_priority(c: char) -> u32 {
    let val: u32 = c as u32;
    return if val > 96 {
        val - 96
    } else {
        val - 38
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input/real") {
        let mut sum_of_priorities: u32 = 0;
        for line in lines {
            if let Ok(l) = line {
                let len = l.chars().count();
                let (str1, str2) = l.split_at(len / 2);

                for char in str1.chars() {
                    if str2.contains(char) {
                        sum_of_priorities += get_priority(char);
                        break;
                    }
                }
            }
        }
        println!("[Part 1] Sum of priorities: {}", sum_of_priorities);
    }

    // Part 2
    if let Ok(lines) = read_lines("./input/real") {
        let mut sum_of_priorities_p2: u32 = 0;
        let mut result_vec: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(l) = line {
                if result_vec.len() != 3 {
                    result_vec.push(l);
                }
                if result_vec.len() == 3 {
                    for char in result_vec[0].chars() {
                        if (result_vec[1].contains(char)) & (result_vec[2].contains(char)) {
                            sum_of_priorities_p2 += get_priority(char);
                            break;
                        }
                    }
                    result_vec.clear();
                }
            }
        }
        println!("[Part 2] Sum of priorities: {}", sum_of_priorities_p2);
    }
}
