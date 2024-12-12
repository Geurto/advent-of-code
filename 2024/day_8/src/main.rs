use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use env_logger::Builder;
use log::{debug, info};

fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes_set: HashSet<(usize, usize)> = HashSet::new();

    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let height = lines.len();
    let width = lines.get(0).map_or(0, |line| line.len());

    for (i, line) in lines.iter().enumerate() {
        println!("{}", line.clone());
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push((i, j));
            }
        }
    }

    for (_, positions) in antennas.iter() {
        for &antenna in positions {
            let other_positions = positions.iter().filter(|&&p| p != antenna);

            for &other_antenna in other_positions {
                calculate_antinodes(antenna, other_antenna, width, height, &mut antinodes_set);
            }
        }
    }

    info!("City map has {} antinodes", antinodes_set.len());
    Ok(())
}

fn calculate_antinodes(
    a: (usize, usize),
    b: (usize, usize),
    width: usize,
    height: usize,
    set: &mut HashSet<(usize, usize)>,
) {
    // vector from a to b
    let d_row: i16 = b.0 as i16 - a.0 as i16;
    let d_col: i16 = b.1 as i16 - a.1 as i16;

    // one antinode at a - d
    let antinode_1 = (a.0 as i16 - d_row, a.1 as i16 - d_col);
    if antinode_1.0 >= 0
        && antinode_1.0 < height as i16
        && antinode_1.1 >= 0
        && antinode_1.1 < width as i16
    {
        set.insert((antinode_1.0 as usize, antinode_1.1 as usize));
    }

    // one antinode at b + d
    let antiode_2 = (b.0 as i16 + d_row, b.1 as i16 + d_col);
    if antiode_2.0 >= 0
        && antiode_2.0 < height as i16
        && antiode_2.1 >= 0
        && antiode_2.1 < width as i16
    {
        set.insert((antiode_2.0 as usize, antiode_2.1 as usize));
    }
}
