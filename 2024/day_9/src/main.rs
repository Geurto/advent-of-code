use std::{fmt, fs, usize};

use env_logger::Builder;
use log::{debug, error, info};

#[derive(Clone, Copy, Debug, PartialEq)]
enum DiskContent {
    File { id: usize },
    FreeSpace,
}

impl DiskContent {
    fn to_digit(&self) -> usize {
        match self {
            DiskContent::File { id } => *id,
            DiskContent::FreeSpace => 0,
        }
    }
}

impl fmt::Display for DiskContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskContent::File { id } => write!(f, "{}", id),
            DiskContent::FreeSpace => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct DiskMap {
    disk_layout: Vec<DiskContent>,
}

impl DiskMap {
    fn new(contents: &str) -> Self {
        let mut disk_layout = Vec::new();
        let mut is_file = true;
        let mut file_id = 0;
        for char in contents.chars() {
            match char.to_digit(10) {
                Some(count) => {
                    let count = count as usize;
                    if is_file {
                        disk_layout.extend(
                            std::iter::repeat(DiskContent::File { id: file_id }).take(count),
                        );
                        file_id += 1;
                    } else {
                        disk_layout.extend(std::iter::repeat(DiskContent::FreeSpace).take(count));
                    }
                    is_file = !is_file;
                }
                None => {
                    error!("Invalid character: {}", char);
                }
            }
        }

        DiskMap { disk_layout }
    }

    fn print_layout(&self) -> String {
        self.disk_layout
            .iter()
            .map(|content| format!("| {} ", content))
            .collect::<String>()
            + "|"
    }

    fn defrag(&mut self) {
        let mut free_index = 0;
        let mut file_index = self.disk_layout.len().saturating_sub(1);

        while free_index < self.disk_layout.len()
            && self.disk_layout[free_index] != DiskContent::FreeSpace
        {
            free_index += 1;
        }
        while file_index > free_index && self.disk_layout[file_index] == DiskContent::FreeSpace {
            file_index = file_index.saturating_sub(1);
        }

        while free_index < file_index {
            self.disk_layout.swap(free_index, file_index);

            free_index += 1;
            while free_index < self.disk_layout.len()
                && self.disk_layout[free_index] != DiskContent::FreeSpace
            {
                free_index += 1;
            }

            if file_index == 0 {
                break;
            }

            file_index -= 1;
            while file_index > free_index && self.disk_layout[file_index] == DiskContent::FreeSpace
            {
                if file_index == 0 {
                    break;
                }
                file_index -= 1;
            }
        }
    }

    fn calculate_checksum(&self) -> usize {
        self.disk_layout
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let f = c.to_digit();
                debug!("{}, {} : {}", i, f, i * f);
                i * f
            })
            .sum::<usize>()
    }
}

fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let contents = fs::read_to_string("data/input")?.trim().to_string();
    debug!("Loaded content:\n{}", contents);

    let mut disk_map = DiskMap::new(&contents);
    debug!("Initial layout:\n{}", disk_map.print_layout());
    disk_map.defrag();
    debug!("Defragged layout:\n{}", disk_map.print_layout());
    info!("Checksum: {}", disk_map.calculate_checksum());

    Ok(())
}
