use regex::Regex;

fn extract_first_number(input: String) -> Option<String> {
    let re = Regex::new(r"\d").expect("Invalid regex");

    if let Some(captures) = re.find(&*input) {
        let number_str = captures.as_str();
        let start_index = captures.start();
        Some(number_str.parse().unwrap())
    } else {
        None
    }
}

fn replace_substring_with_number(input: &str) -> String {
    let mut input_modified = input.to_string();
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").expect("Invalid regex");

    while let Some(captures) = re.find(&*input_modified) {
        let number_str = captures.as_str();
        let start_index = captures.start();
        let end_index = captures.end();
        let number = match_number(number_str).expect("Invalid number");
        input_modified.replace_range(start_index..end_index, &*number.to_string());
    }
    input_modified
}

fn match_number(input: &str) -> Option<i32> {
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (index, number) in numbers.iter().enumerate() {
        if input == *number {
            return Some(index as i32);
        }
    }
    None
}

fn main() {
    // let input_file = std::fs::read_to_string("input/dummy").expect("Unable to read file");
    let input_file = std::fs::read_to_string("input/part-2").expect("Unable to read file");
    let mut sum = 0;

    for line in input_file.lines() {
        let line_modified = replace_substring_with_number(line);
        let number_fw = extract_first_number(line_modified.clone()).expect("No number found");

        let line_reverse = line_modified.clone().chars().rev().collect::<String>();
        let number_rev = extract_first_number(line_reverse).expect("No number found");


        let combined_number = format!("{}{}", number_fw, number_rev)
            .parse::<i32>()
            .expect("Invalid number");
        sum += combined_number;
        println!("Line: {}. \nModified line: {}. \nNumbers: {} + {} = {}", line, line_modified, number_fw, number_rev, combined_number);
    }

    println!("Sum: {:?}", sum);
}
