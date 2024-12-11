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
    position: (usize, usize),
    direction: Direction,
    left_area: bool,
    path: HashSet<(usize, usize)>,
    path_directional: HashSet<((usize, usize), Direction)>,
}

impl Guard {
    fn default() -> Self {
        Guard::new((0, 0), Direction::Up)
    }

    fn new(position: (usize, usize), direction: Direction) -> Self {
        Guard {
            position,
            direction,
            left_area: false,
            path: HashSet::new(),
            path_directional: HashSet::new(),
        }
    }

    fn step(&mut self, map: &mut Map) {
        // up on map means lower in index
        let (row, col) = self.position;
        let (new_row, new_col) = match self.direction {
            Direction::Up => (row.wrapping_sub(1), col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.wrapping_sub(1)),
        };

        // check bounds
        if new_col >= map.width || new_row >= map.height {
            self.left_area = true;
            return;
        }

        // check for obstacle
        if map.cells[new_row][new_col].is_obstacle {
            self.change_direction();
            return;
        }

        map.cells[new_row][new_col].add_guard();
        map.cells[row][col].remove_guard();
        map.cells[new_row][new_col].cross(self.direction.clone());

        self.position = (new_row, new_col);
        self.path.insert(self.position);
        self.path_directional
            .insert((self.position, self.direction.clone()));
    }

    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn check_loop(&self) -> bool {
        let next_position = match self.direction {
            Direction::Up => (self.position.0.wrapping_sub(1), self.position.1),
            Direction::Right => (self.position.0, self.position.1 + 1),
            Direction::Down => (self.position.0 + 1, self.position.1),
            Direction::Left => (self.position.0, self.position.1.wrapping_sub(1)),
        };

        self.path_directional
            .contains(&(next_position, self.direction.clone()))
    }

    fn to_char(&self) -> &str {
        match self.direction {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "V",
            Direction::Left => "<",
        }
    }

    fn to_direction(&self, c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'V' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
}

#[derive(Clone, Default, PartialEq)]
struct Cell {
    symbol: char,
    is_obstacle: bool,
    has_guard: bool,
    directions_crossed: HashSet<Direction>,
}

impl Cell {
    fn new(value: char, has_guard: bool) -> Self {
        let is_obstacle: bool = value == '#';

        Cell {
            symbol: value,
            is_obstacle,
            has_guard,
            directions_crossed: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.directions_crossed = HashSet::new();
        self.remove_guard();
        self.is_obstacle = self.symbol == '#';
    }

    fn remove_guard(&mut self) {
        self.has_guard = false;
    }
    fn add_guard(&mut self) {
        self.has_guard = true;
    }

    fn cross(&mut self, direction: Direction) {
        self.directions_crossed.insert(direction);
    }

    fn get_char(&mut self) -> char {
        if self.is_obstacle {
            self.symbol = '#';
        } else if self.has_guard {
            self.symbol = 'G';
        } else {
            self.symbol = match (
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

        self.symbol
    }
}

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        Map {
            width,
            height,
            cells,
        }
    }

    fn reset(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.reset();
            }
        }
    }

    fn add_obstacle(&mut self, row: usize, col: usize) {
        self.cells[row][col].symbol = 'O';
        self.cells[row][col].is_obstacle = true;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("data/input")?;
    let reader = BufReader::new(file);

    let mut guard = Guard::default();
    let mut initial_position: (usize, usize) = (0, 0);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let height = lines.len();
    let width = lines.get(0).map_or(0, |line| line.len());

    let mut cells = vec![vec![Cell::default(); width]; height];

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let cell;
            if let Some(direction) = guard.to_direction(ch) {
                cell = Cell::new(ch, true);
                guard.position = (row, col);
                guard.direction = direction;
                initial_position = (row, col);
            } else {
                cell = Cell::new(ch, false);
            }
            cells[row][col] = cell;
        }
    }
    let mut map = Map::new(cells);

    while !guard.left_area {
        guard.step(&mut map);
    }

    println!("Number of unique cells visited: {}", guard.path.len());
    //print(&map, &guard);

    let mut num_loops: u16 = 0;
    for (row, col) in guard.path.iter() {
        map.reset();
        let mut new_guard = Guard::new(initial_position, Direction::Up);

        // add obstacle
        map.add_obstacle(*row, *col);

        // check for loop
        while !new_guard.left_area {
            new_guard.step(&mut map);
            if new_guard.check_loop() {
                num_loops += 1;
                break;
            }
        }
    }

    println!(
        "Number of obstacle placements where guard gets into a loop: {}",
        num_loops
    );

    Ok(())
}

fn print(map: &Map, guard: &Guard) {
    for mut row in map.cells.clone() {
        let line: String = row.iter_mut().map(|cell| cell.get_char()).collect();
        let _ = line.replace("G", guard.to_char());
        println!("{}", line);
    }
}
