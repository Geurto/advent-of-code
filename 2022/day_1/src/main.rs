use std::env;
use std::fs::{File, read};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let workdir = env::current_dir().unwrap();
    println!("{}", workdir.into_os_string().into_string().unwrap());
    if let Ok(lines) = read_lines("./input/real") {
        let mut vec_calories_per_elf: Vec<i32> = Vec::new();
        let mut current_elf_calories: i32 = 0;
        for line in lines {
            if let Ok(l) = line {
                if l.is_empty() {
                    vec_calories_per_elf.push(current_elf_calories);
                    println!("Calories for this elf: {}", current_elf_calories);
                    current_elf_calories = 0;
                } else {
                    let cal: i32 = l.to_string().parse().unwrap();
                    current_elf_calories += cal;
                }
            }
        }
        vec_calories_per_elf.push(current_elf_calories);
        println!("Calories for this elf: {}", current_elf_calories);

        vec_calories_per_elf.sort_by(|a, b| b.cmp(a));

        // Part 1: find max value in list
        let mut it = vec_calories_per_elf.into_iter();
        let max_value: i32 = it.next().unwrap();
        println!("Max calories carried by single elf: {}", max_value);

        // Part 2: find top 3 of list
        let mut top_three_calories: i32 = max_value;
        top_three_calories += it.next().unwrap();
        top_three_calories += it.next().unwrap();
        println!("Calories carried by top 3 elves: {}", top_three_calories);

    }
}