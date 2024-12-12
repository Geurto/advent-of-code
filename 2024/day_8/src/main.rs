use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::{Context, Result};

#[derive(Debug)]
struct CityGrid {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Position>>,
    antinodes: HashSet<Position>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position(usize, usize);

impl CityGrid {
    fn new(input: &str) -> Result<Self> {
        let file = File::open(input).context("Failed to open input file")?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

        let height = lines.len();
        let width = lines.first().map_or(0, |line| line.len());
        let mut grid = Self {
            width,
            height,
            antennas: HashMap::new(),
            antinodes: HashSet::new(),
        };

        // Parse input and populate antennas
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    grid.add_antenna(c, Position(row, col));
                }
            }
        }

        grid.calculate_all_antinodes();
        Ok(grid)
    }

    fn add_antenna(&mut self, antenna_type: char, pos: Position) {
        self.antennas.entry(antenna_type)
            .or_default()
            .push(pos);
    }

    fn calculate_all_antinodes(&mut self) {
        for positions in self.antennas.values() {
            for &pos1 in positions {
                for &pos2 in positions {
                    if pos1 != pos2 {
                        if let Some(new_antinodes) = self.calculate_antinodes(pos1, pos2) {
                            self.antinodes.extend(new_antinodes);
                        }
                    }
                }
            }
        }
    }

    fn calculate_antinodes(&self, a: Position, b: Position) -> Option<Vec<Position>> {
        let Position(row_a, col_a) = a;
        let Position(row_b, col_b) = b;

        // Calculate vector from a to b
        let d_row = row_b as i32 - row_a as i32;
        let d_col = col_b as i32 - col_a as i32;

        let mut result = Vec::new();

        // Calculate potential antinodes
        let candidates = [
            (row_a as i32 - d_row, col_a as i32 - d_col),  // a - d
            (row_b as i32 + d_row, col_b as i32 + d_col),  // b + d
        ];

        // Filter valid positions
        for (row, col) in candidates {
            if self.is_in_bounds(row, col) {
                result.push(Position(row as usize, col as usize));
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn is_in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32 && col >= 0 && col < self.width as i32
    }
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let city_grid = CityGrid::new("data/input")?;
    println!("City map has {} antinodes", city_grid.antinodes.len());
    Ok(())
}