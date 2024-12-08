use std::fs::File;
use std::io::{BufRead, BufReader};

struct LetterGrid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    xmas_hits: u32,
    cross_mas_hits: u32,
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

    const CROSS_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)]; // A from M

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
            cross_mas_hits: 0,
        }
    }

    fn is_in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32 && col >= 0 && col < self.width as i32
    }

    fn check_letter(&self, row: i32, col: i32, desired: char) -> bool {
        self.is_in_bounds(row, col) && self.grid[row as usize][col as usize] == desired
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

        for &(dy, dx) in Self::DIRECTIONS.iter() {
            let m_row = base_row + dy;
            let m_col = base_col + dx;

            if !self.check_letter(m_row, m_col, 'M') {
                continue;
            }

            let a_row = m_row + dy;
            let a_col = m_col + dx;

            if !self.check_letter(a_row, a_col, 'A') {
                continue;
            }

            let s_row = a_row + dy;
            let s_col = a_col + dx;

            if !self.check_letter(s_row, s_col, 'S') {
                continue;
            }

            self.xmas_hits += 1;
        }
    }

    fn find_cross_mas_patterns(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == 'M' {
                    self.check_cross_mas_from_position(row, col);
                }
            }
        }
    }

    fn check_cross_mas_from_position(&mut self, row: usize, col: usize) {
        let base_row = row as i32;
        let base_col = col as i32;

        for &(dy, dx) in Self::CROSS_DIRECTIONS.iter() {
            // check for A
            let a_row = base_row + dy;
            let a_col = base_col + dx;

            if !self.check_letter(a_row, a_col, 'A') {
                continue;
            }

            let ms_row = base_row + 2 * dy;
            let ms_col = base_col + 2 * dx;

            // need an S across
            if !self.check_letter(ms_row, ms_col, 'S') {
                continue;
            }

            // check letter on same row
            if self.check_letter(base_row, ms_col, 'M') {
                // need an S on the same column
                if !self.check_letter(ms_row, base_col, 'S') {
                    continue;
                }
                self.cross_mas_hits += 1;
            } else if self.check_letter(base_row, ms_col, 'S') {
                // need an M on the same column
                if !self.check_letter(ms_row, base_col, 'M') {
                    continue;
                }
                self.cross_mas_hits += 1;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut letter_grid = LetterGrid::new(lines);

    // Part 1
    letter_grid.find_xmas_patterns();
    println!("Found {} XMAS patterns", letter_grid.xmas_hits);

    // Part 2: checking every M means we find 2N patterns
    letter_grid.find_cross_mas_patterns();
    println!(
        "Found {} crossed MAS patterns",
        letter_grid.cross_mas_hits / 2
    );

    Ok(())
}
