use regex::Regex;

fn main() {
    let input_file = std::fs::read_to_string("input/dummy").expect("Unable to read file");
    let mut game_number = 1;
    for line in input_file.lines() {
        // remove Game #x: from line
        let line = line.replace(&format!("Game {}: ", game_number), "");
        let draws = line.split("; ").collect::<Vec<&str>>();
        game_number += 1;

    }
}
