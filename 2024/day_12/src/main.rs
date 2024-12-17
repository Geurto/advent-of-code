use std::collections::HashMap;
use std::fs;

use env_logger::Builder;
use log::{debug, info};

struct Garden {
    width: usize,
    height: usize,
    plants: Vec<Vec<String>>,
    plots: Vec<GardenPlot>,
    costs_per_type: HashMap<String, usize>,
}

impl Garden {
    fn new(contents: &str) -> Self {
        let rows = contents.split("\n").collect::<Vec<&str>>();
        let width = rows[0].len();
        let height = rows.len();

        let costs_per_type = HashMap::<String, usize>::new();
        let plots = Vec::<GardenPlot>::new();

        let mut plants = Vec::<Vec<String>>::new();
        for (i, row) in rows.iter().enumerate() {
            plants.push(Vec::<String>::new());
            for cell in row.chars() {
                plants[i].push(cell.to_string());
            }
        }

        Self {
            width,
            height,
            plants,
            plots,
            costs_per_type,
        }
    }

    fn survey(&mut self) {
        let mut visited = vec![vec![false; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                if !visited[y][x] {
                    let plant_type = &self.plants[y][x];
                    let mut plot = GardenPlot::new(plant_type.clone(), 0, 0);
                    self.explore(x, y, &mut visited, &mut plot);
                    self.plots.push(plot);
                }
            }
        }
    }

    fn explore(&mut self, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, plot: &mut GardenPlot) {
        if visited[y][x] {
            return;
        }

        visited[y][x] = true;
        plot.add_plant(x, y);
        let neighbours = self.get_neighbours_of_same_type(x, y);
        let mut same_type_neighbours = 0;

        for (nx, ny) in neighbours {
            same_type_neighbours += 1;
            self.explore(nx, ny, visited, plot);
        }

        plot.perimeter += 4 - same_type_neighbours;
        plot.area += 1;
    }

    fn get_neighbours_of_same_type(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::<(usize, usize)>::new();
        let plant_type = &self.plants[y][x];

        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if self.plants[ny][nx] == *plant_type {
                    neighbours.push((nx, ny));
                }
            }
        }

        neighbours
    }

    fn calculate_costs(&mut self) {
        for (i, plot) in self.plots.iter().enumerate() {
            let cost = plot.area * plot.perimeter;
            debug!(
                "# {}. Plant: {}, Area: {}, Perimeter: {}, Cost: {}",
                i, plot.plant_type, plot.area, plot.perimeter, cost
            );
            match self.costs_per_type.get_mut(&plot.plant_type) {
                Some(plant_map) => *plant_map += cost,
                None => {
                    self.costs_per_type.insert(plot.plant_type.clone(), cost);
                }
            }
        }

        let total_costs = self
            .costs_per_type
            .iter()
            .fold(0, |acc, (_, cost)| acc + cost);
        info!("Total costs: {}", total_costs);
    }
}

struct GardenPlot {
    plant_type: String,
    plants: Vec<(usize, usize)>,
    area: usize,
    perimeter: usize,
}

impl GardenPlot {
    fn new(plant_type: String, area: usize, perimeter: usize) -> Self {
        Self {
            plant_type,
            plants: Vec::<(usize, usize)>::new(),
            area,
            perimeter,
        }
    }

    fn add_plant(&mut self, x: usize, y: usize) {
        self.plants.push((x, y));
    }
}

fn main() {
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    let contents = fs::read_to_string("data/input")
        .expect("Something went wrong reading the file")
        .trim()
        .to_string();
    info!("Plant garden:\n{}", contents);

    let mut garden = Garden::new(&contents);
    garden.survey();
    garden.calculate_costs();
}
