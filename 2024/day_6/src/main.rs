use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq)]
struct Guard {
    direction: Direction,
}

impl Guard {
    fn to_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'V',
            Direction::Left => '<',
        }
    }
}

#[derive(Clone, Default, PartialEq)]
struct Cell {
    is_obstacle: bool,
    guard: Option<Guard>,
    directions_crossed: HashSet<Direction>,
}

impl Cell {
    fn new(value: char) -> Self {
        let is_obstacle: bool = value == '#';

        Cell {
            is_obstacle,
            guard: None,
            directions_crossed: HashSet::new(),
        }
    }

    fn cross(&mut self, direction: Direction) {
        self.directions_crossed.insert(direction);
    }

    fn get_char(&self) -> char {
        if self.is_obstacle {
            return '#';
        }

        if let Some(guard) = &self.guard {
            return guard.to_char();
        }

        match (
            self.directions_crossed.contains(&Direction::Left)
                || self.directions_crossed.contains(&Direction::Right),
            self.directions_crossed.contains(&Direction::Up)
                || self.directions_crossed.contains(&Direction::Down),
        ) {
            (true, true) => '+',
            (true, false) => '-',
            (false, true) => '|',
            (false, false) => '.',
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    guard_position: Option<(usize, usize)>,
    guard_direction: Direction,
    completed: bool,
}

impl Map {
    fn new(lines: Vec<String>) -> Self {
        let height = lines.len();
        let width = lines.get(0).map_or(0, |line| line.len());

        let mut cells = vec![vec![Cell::default(); width]; height];
        let mut guard_position = None;
        let mut guard_direction: Direction = Direction::Up;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let mut cell = Cell::new(ch);
                if let Some(direction) = get_guard_direction(ch) {
                    cell.guard = Some(Guard {
                        direction: direction.clone(),
                    });
                    guard_position = Some((row, col));
                    guard_direction = direction;
                }
                cells[row][col] = cell;
            }
        }

        Map {
            width,
            height,
            cells,
            guard_position,
            guard_direction,
            completed: false,
        }
    }

    fn print(&self) {
        for row in &self.cells {
            let line: String = row.iter().map(|cell| cell.get_char()).collect();
            println!("{}", line);
        }
    }

    fn move_guard(&mut self) {
        // up on map means lower in index
        if let Some((row, col)) = self.guard_position {
            let (new_row, new_col) = match self.guard_direction {
                Direction::Up => (row.wrapping_sub(1), col),
                Direction::Right => (row, col + 1),
                Direction::Down => (row + 1, col),
                Direction::Left => (row, col.wrapping_sub(1)),
            };

            // check bounds
            if new_col >= self.width || new_row >= self.height {
                self.completed = true;
                return;
            }

            // check for obstacle
            if self.cells[new_row][new_col].is_obstacle {
                self.change_direction();
                return;
            }

            self.cells[row][col].guard = None;
            self.cells[new_row][new_col].guard = Some(Guard {
                direction: self.guard_direction.clone(),
            });
            self.cells[new_row][new_col].cross(self.guard_direction.clone());

            self.guard_position = Some((new_row, new_col));
        }
    }

    fn change_direction(&mut self) {
        self.guard_direction = match self.guard_direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_total_visited_cells(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| !cell.directions_crossed.is_empty())
            .count()
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("data/dummy")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut map = Map::new(lines);

    while !map.completed {
        map.move_guard();
    }

    println!(
        "Number of unique cells visited: {}",
        map.get_total_visited_cells()
    );

    map.print();

    Ok(())
}

fn get_guard_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'V' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        _ => None,
    }
}
