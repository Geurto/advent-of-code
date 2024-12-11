use std::fs::File;
use std::io::{BufRead, BufReader};

struct Equation {
    result: u64,
    factors: Vec<u16>,
}

impl Equation {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split(": ").collect();
        let result = split[0].parse::<u64>().unwrap();
        let mut factors: Vec<u16> = split[1]
            .split(" ")
            .map(|s| s.parse::<u16>().unwrap())
            .collect();

        println!("Factors: {:?}, result: {}", factors, result);
        Equation { result, factors }
    }

    fn solve(&mut self) {
        for &f in self.factors.iter().rev() {
            if self.result % f as u64 == 0 {
                println!("{} is a factor of {}", f, self.result);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("data/dummy")?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let mut eq = Equation::new(&line);
        eq.solve();
    }

    Ok(())
}
