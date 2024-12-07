use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Expression {
    Mul(i32, i32),
    Do,
    Dont,
}

#[derive(PartialEq)]
enum State {
    On,
    Off,
}

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let mut sum: i64 = 0;
    let mut sum_with_state: i64 = 0;
    let mut state: State = State::On;
    for line in reader.lines() {
        let line = line?;

        // Part 1
        let pairs = extract_multiplication_values(&line);
        sum += multiply_and_sum(&pairs) as i64;

        // Part 2
        let expressions = find_expressions(&line);
        for expr in expressions {
            match expr {
                Expression::Mul(x, y) => {
                    if state == State::On {
                        sum_with_state += (x * y) as i64;
                    }
                }
                Expression::Do => state = State::On,
                Expression::Dont => state = State::Off,
            }
        }
    }

    println!("Computed sum {}", sum);
    println!("Computed sum with on/off switching {}", sum_with_state);

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
fn find_expressions(input: &str) -> Vec<Expression> {
    let re = Regex::new(r"(?P<mul>mul\s*\((\d+)\s*,\s*(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))")
        .unwrap();

    let mut expressions = Vec::new();

    for caps in re.captures_iter(input) {
        if caps.name("mul").is_some() {
            let x = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            expressions.push(Expression::Mul(x, y));
        } else if caps.name("do").is_some() {
            expressions.push(Expression::Do);
        } else if caps.name("dont").is_some() {
            expressions.push(Expression::Dont);
        }
    }

    expressions
}

fn multiply_and_sum(pairs: &[(i32, i32)]) -> i32 {
    pairs.iter().map(|(x, y)| x * y).sum()
}
