use std::collections::HashSet;

use env_logger::Builder;
use log::{debug, info};

const DATA: &str = include_str!("../data/input");

#[derive(Clone, Copy)]
struct Cell {
    height: usize,
    visited: bool,
}

impl Cell {
    fn new(height: usize) -> Self {
        Cell {
            height,
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn reset(&mut self) {
        self.visited = false;
    }
}

struct TopographicMap {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    trailheads: Vec<((usize, usize), usize)>,
    reachable_peaks: HashSet<(usize, usize)>,
}

impl TopographicMap {
    fn new(height_map: &str) -> Self {
        let mut cells = Vec::new();
        let mut trailheads = Vec::new();
        let reachable_peaks = HashSet::new();
        let height = height_map.lines().count();
        let width = height_map.lines().next().unwrap().chars().count();

        for (y, line) in height_map.split('\n').enumerate() {
            let mut row: Vec<Cell> = Vec::new();
            for (x, char) in line.chars().enumerate() {
                let cell = Cell::new(char.to_digit(10).unwrap() as usize);
                row.push(cell);
                if cell.height == 0 {
                    trailheads.push(((x, y), 0));
                }
            }
            cells.push(row);
        }

        TopographicMap {
            cells,
            width,
            height,
            trailheads,
            reachable_peaks,
        }
    }

    fn reset(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.reset();
            }
        }
    }

    fn explore_all(&mut self) {
        let trailheads = std::mem::take(&mut self.trailheads);
        let mut total_score = 0;
        for ((x, y), mut score) in trailheads {
            self.reset();
            score = self.explore(x, y);
            total_score += score;
            debug!("Trailhead at [{}, {}] has a score of {}", x, y, score);
            debug!("Reachable peaks: {:?}", self.reachable_peaks);
        }

        info!("Total score: {}", total_score);
    }

    fn explore(&mut self, x0: usize, y0: usize) -> usize {
        let mut cells_to_visit = vec![(x0, y0)];
        let mut score = 0;
        while let Some((x, y)) = cells_to_visit.pop() {
            let cell = &mut self.cells[y][x];
            cell.visit();
            if cell.height == 9 {
                debug!("Reached peak at [{}, {}] from [{}, {}]", x, y, x0, y0);
                self.reachable_peaks.insert((x, y));
                score += 1;
                continue;
            }

            let viable_neighbours = self.get_viable_neighbours(x, y);
            for (nx, ny) in viable_neighbours {
                cells_to_visit.push((nx, ny));
            }
        }
        score
    }

    fn get_viable_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let current_height = self.cells[y][x].height;
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut viable_neighbours = Vec::new();

        for (dx, dy) in directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x < 0 || new_x >= self.width as isize {
                continue;
            }
            if new_y < 0 || new_y >= self.height as isize {
                continue;
            }

            let next_cell = self.cells[new_y as usize][new_x as usize];
            if next_cell.height != current_height + 1 {
                continue;
            }
            if next_cell.visited {
                continue;
            }
            viable_neighbours.push((new_x as usize, new_y as usize));
        }

        viable_neighbours
    }
}

fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let mut map = TopographicMap::new(DATA);
    info!("\n---- MAP ----\n{}", DATA);
    debug!("Trailheads: {:?}", map.trailheads);

    map.explore_all();
}
