use std::collections::HashMap;
use std::fs;

use env_logger::Builder;
use log::{debug, info};

struct Garden {
    plots: HashMap<String, GardenPlot>,
    width: usize,
    height: usize,
    layout: Vec<Vec<String>>,
}

impl Garden {
    fn new(contents: &str) -> Self {
        let rows = contents.split("\n").collect::<Vec<&str>>();
        let width = rows[0].len();
        let height = rows.len();

        let mut plots = HashMap::<String, GardenPlot>::new();
        let mut layout = Vec::<Vec<String>>::new();

        for (i, row) in rows.iter().enumerate() {
            layout.push(Vec::<String>::new());
            for cell in row.chars() {
                layout[i].push(cell.to_string());
                let plant_type = cell.to_string();
                let plot = plots.get(&plant_type);
                if plot.is_none() {
                    plots.insert(
                        plant_type.clone(),
                        GardenPlot::new(plant_type.clone(), 0, 0),
                    );
                }
                let plot = plots.get_mut(&plant_type).unwrap();
                plot.area += 1;
                plot.perimeter += 4 - 2;
            }
        }

        Self {
            plots,
            width,
            height,
            layout,
        }
    }

    fn survey(&mut self) {}

    fn get_number_of_same_neighbours(&self, x: usize, y: usize) -> usize {}
}

struct GardenPlot {
    plant_type: String,
    area: usize,
    perimeter: usize,
}

impl GardenPlot {
    fn new(plant_type: String, area: usize, perimeter: usize) -> Self {
        Self {
            plant_type,
            area,
            perimeter,
        }
    }
}

fn main() {
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    let contents = fs::read_to_string("data/dummy").expect("Something went wrong reading the file");
    info!("Plant garden:\n{}", contents);

    let mut garden = Garden::new(&contents);
    garden.survey();
}
