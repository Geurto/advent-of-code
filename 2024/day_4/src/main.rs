use std::fs::File;
use std::io::{BufRead, BufReader};

struct LetterGrid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    xmas_hits: u32,
}

impl LetterGrid {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn new(lines: Vec<String>) -> Self {
        let height = lines.len();
        let width = lines.get(0).map_or(0, |line| line.len());

        let mut grid = vec![vec![' '; width]; height];

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                grid[row][col] = ch;
            }
        }

        println!(
            "Making LetterGrid with {} rows and {} columns",
            height, width
        );

        LetterGrid {
            grid,
            width,
            height,
            xmas_hits: 0,
        }
    }

    fn is_in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32 && col >= 0 && col < self.width as i32
    }

    fn get_letter(&self, row: i32, col: i32) -> Option<&char> {
        if self.is_in_bounds(row, col) {
            Some(&self.grid[row as usize][col as usize])
        } else {
            None
        }
    }

    fn find_xmas_patterns(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == 'X' {
                    self.check_xmas_from_position(row, col);
                }
            }
        }
    }

    fn check_xmas_from_position(&mut self, row: usize, col: usize) {
        let base_row = row as i32;
        let base_col = col as i32;

        for &(dx, dy) in Self::DIRECTIONS.iter() {
            let m_row = base_row + dx;
            let m_col = base_col + dy;

            let m_letter = if let Some(x) = self.get_letter(m_row, m_col) {
                x
            } else {
                continue;
            };
            if *m_letter != 'M' {
                continue;
            }

            let a_row = m_row + dx;
            let a_col = m_col + dy;

            let a_letter = if let Some(x) = self.get_letter(a_row, a_col) {
                x
            } else {
                continue;
            };
            if *a_letter != 'A' {
                continue;
            }

            let s_row = a_row + dx;
            let s_col = a_col + dy;

            let s_letter = if let Some(x) = self.get_letter(s_row, s_col) {
                x
            } else {
                continue;
            };
            if *s_letter != 'S' {
                continue;
            }

            self.xmas_hits += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut letter_grid = LetterGrid::new(lines);

    letter_grid.find_xmas_patterns();

    println!("Found {} XMAS patterns", letter_grid.xmas_hits);

    Ok(())
}
