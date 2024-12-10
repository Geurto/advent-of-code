use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Default)]
struct Position {
    x: i16,
    y: i16,
}

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
    visited_cells: Vec<bool>,
    guard_position: Position,
    guard_direction: Direction,
    completed: bool,
}

impl Map {
    fn new(lines: Vec<String>) -> Self {
        let height = lines.len();
        let width = lines.get(0).map_or(0, |line| line.len());

        let mut cells = vec![vec![' '; width]; height];
        let mut visited_cells = vec![false; width * height];
        let mut guard_position = Position::default();
        let mut guard_direction: Direction = Direction::Up;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                cells[row][col] = ch;
                if let Some(direction) = get_guard_direction(ch) {
                    guard_position.x = col as i16;
                    guard_position.y = row as i16;
                    guard_direction = direction;
                }
            }
        }

        visited_cells[guard_position.y as usize * width + guard_position.x as usize] = true;

        Map {
            width,
            height,
            cells,
            visited_cells,
            guard_position,
            guard_direction,
            completed: false,
        }
    }

    fn move_guard(&mut self) {
        // up on map means lower in index
        let mut new_position = self.guard_position.clone();
        match self.guard_direction {
            Direction::Up => {
                new_position.y -= 1;
            }
            Direction::Right => {
                new_position.x += 1;
            }
            Direction::Down => {
                new_position.y += 1;
            }
            Direction::Left => {
                new_position.x -= 1;
            }
        }

        // check bounds
        if new_position.x < 0
            || new_position.x >= self.width as i16
            || new_position.y < 0
            || new_position.y >= self.height as i16
        {
            self.completed = true;
            return;
        }

        // check for obstacle
        if self.is_obstacle(new_position.clone()) {
            self.change_direction();
            return;
        }

        self.guard_position = new_position;
        self.visited_cells
            [self.guard_position.y as usize * self.width + self.guard_position.x as usize] = true;
    }

    fn change_direction(&mut self) {
        match self.guard_direction {
            Direction::Up => {
                self.guard_direction = Direction::Right;
            }
            Direction::Right => {
                self.guard_direction = Direction::Down;
            }
            Direction::Down => {
                self.guard_direction = Direction::Left;
            }
            Direction::Left => {
                self.guard_direction = Direction::Up;
            }
        }
        println!("Changed direction to {:?}", self.guard_direction);
    }

    fn is_obstacle(&self, position: Position) -> bool {
        self.cells[position.y as usize][position.x as usize] == '#'
    }

    fn get_total_visited_cells(&self) -> u16 {
        self.visited_cells
            .iter()
            .map(|b| match b {
                false => 0,
                true => 1,
            })
            .sum::<u16>()
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
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
