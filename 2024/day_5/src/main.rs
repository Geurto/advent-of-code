use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Rule {
    first: u16,
    second: u16,
}

impl Rule {
    fn new(first: u16, second: u16) -> Self {
        Rule { first, second }
    }

    fn get_indices(&self, pages: &[u16]) -> (Option<usize>, Option<usize>) {
        (
            pages.iter().position(|&x| x == self.first),
            pages.iter().position(|&x| x == self.second),
        )
    }

    fn check_update(&self, pages: &[u16]) -> Option<bool> {
        let (first_index, second_index) = self.get_indices(pages);

        match (first_index, second_index) {
            (Some(f), Some(s)) => Some(f < s),
            _ => None,
        }
    }

    fn fix(&self, pages: &mut [u16]) {
        if let (Some(first_index), Some(second_index)) = self.get_indices(pages) {
            pages.swap(first_index, second_index);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ordering_rules: Vec<Rule> = Vec::new();
    let mut sum_of_middle_pages: u32 = 0;
    let mut sum_of_fixed_middle_pages: u32 = 0;

    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if line.contains('|') {
            let pages = line.split('|').map(|s| s.parse().unwrap()).collect();
            handle_ordering_rule(pages, &mut ordering_rules);
        } else if line.contains(',') {
            let mut pages: Vec<u16> = line.split(',').map(|s| s.parse().unwrap()).collect();
            if let Some(middle_page) = handle_update(pages.clone(), &ordering_rules) {
                sum_of_middle_pages += middle_page as u32;
            } else if let Some(middle_page_fixed) = fix_pages(&mut pages, &ordering_rules) {
                sum_of_fixed_middle_pages += middle_page_fixed as u32;
            }
        }
    }

    println!(
        "Sum of middle page numbers of correct updates: {}",
        sum_of_middle_pages
    );

    println!("Sum of fixed middle pages: {}", sum_of_fixed_middle_pages);

    Ok(())
}

fn handle_ordering_rule(pages: Vec<u16>, ordering_rules: &mut Vec<Rule>) {
    ordering_rules.push(Rule::new(pages[0], pages[1]));
}

fn handle_update(pages: Vec<u16>, ordering_rules: &Vec<Rule>) -> Option<u16> {
    for rule in ordering_rules {
        if let Some(false) = rule.check_update(&pages) {
            return None;
        }
    }
    pages.get(pages.len() / 2).copied()
}

fn fix_pages(pages: &mut [u16], ordering_rules: &Vec<Rule>) -> Option<u16> {
    if handle_update(pages.to_vec(), ordering_rules).is_some() {
        return None;
    }

    loop {
        let mut fixed = false;

        for rule in ordering_rules {
            if let Some(false) = rule.check_update(pages) {
                rule.fix(pages);
                fixed = true;
            }
        }

        if let Some(middle_page) = handle_update(pages.to_vec(), ordering_rules) {
            return Some(middle_page);
        }

        if !fixed {
            break;
        }
    }

    None
}
