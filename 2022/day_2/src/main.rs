use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_hand_score_p1(rps: &str) -> i64 {
    return if rps == "X" {
        1
    } else if rps == "Y" {
        2
    } else { 3 }
}

fn get_result_score_p1(rps: &Vec<&str>) -> i64 {
    return if rps[0] == "A" {  // Rock
        if rps[1] == "X" {  // Rock
            3
        } else if rps[1] == "Y" { // Paper
            6
        } else { 0 } // Scissors
    } else if rps[0] == "B" { // Paper
        if rps[1] == "X" { // Rock
            0
        } else if rps[1] == "Y" { // Paper
            3
        } else { 6 } // Scissors
    } else {  // Scissors
        if rps[1] == "X" { //Rock
            6
        } else if rps[1] == "Y" { // Paper
            0
        } else { 3 }  // Scissors
    }
}

fn get_result_score_p2(rps: &str) -> i64 {
    return if rps == "X" { // lose
        0
    } else if rps == "Y" { // draw
        3
    } else { 6 } // win
}

fn get_hand_score_p2(rps: &Vec<&str>) -> i64 {
    return if rps[0] == "A" {  // Rock
        if rps[1] == "X" {  // Lose - Scissor
            3
        } else if rps[1] == "Y" { // Draw - Rock
            1
        } else { 2 } // Win - Paper
    } else if rps[0] == "B" { // Paper
        if rps[1] == "X" { // Lose - Rock
            1
        } else if rps[1] == "Y" { // Draw - Paper
            2
        } else { 3 } // Win - Scissors
    } else {  // Scissors
        if rps[1] == "X" { // Lose - Paper
            2
        } else if rps[1] == "Y" { // Draw - Scissors
            3
        } else { 1 }  // Win - Rock
    }
}

fn main() {
    let workdir = env::current_dir().unwrap();
    println!("{}", workdir.into_os_string().into_string().unwrap());
    if let Ok(lines) = read_lines("./input/real") {
        let mut score_part_one: i64 = 0;
        let mut score_part_two: i64 = 0;
        for line in lines {
            if let Ok(l) = line {
                if !l.is_empty() {
                    let result_vec: Vec<&str> = l.split(" ").collect();

                    // Part 1
                    score_part_one += get_hand_score_p1(result_vec[1]);
                    score_part_one +=  get_result_score_p1(&result_vec);

                    // Part 2
                    score_part_two += get_result_score_p2(result_vec[1]);
                    score_part_two += get_hand_score_p2(&result_vec);
                }
            }
        }
        println!("[Part 1] Total expected score: {}", score_part_one);
        println!("[Part 2] Total expected score: {}", score_part_two);
    }
}
