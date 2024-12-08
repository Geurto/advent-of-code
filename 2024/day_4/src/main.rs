use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data/dummy")?;
    let reader = BufReader::new(file);

    // Collect lines into a vector to avoid consuming the reader
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let num_lines = lines.len();

    let mut line_size = 0;
    let mut letters = Vec::new(); // Initialize letters

    let mut index = 0;

    for line in lines {
        println!("Read line {:?}", line);
        if line_size == 0 {
            line_size = line.len();
            letters = vec![vec!['-'; line_size]; num_lines]; // Initialize with default char
        }

        for ch in line.chars() {
            let row = index / line_size;
            let col = index % line_size;
            letters[row][col] = ch; // Store the character itself
            println!("[{}][{}]: {}", row, col, ch);
            index += 1;
        }
    }

    Ok(())
}
