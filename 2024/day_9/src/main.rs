use std::{fs, usize};

use env_logger::Builder;
use log::{debug, error, info};

#[derive(Debug)]
struct DiskMap {
    disk_layout: Vec<char>,
}

impl DiskMap {
    const FILE_IDS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn new(contents: &str) -> Self {
        let mut disk_str: String = "".to_string();
        let mut is_file = true;
        let mut file_id = 0;
        for char in contents.chars() {
            if is_file {
                match char.to_digit(10) {
                    Some(k) => {
                        let id = Self::FILE_IDS.chars().nth(file_id).unwrap();
                        disk_str.push_str(&id.to_string().repeat(k as usize));
                        file_id += 1;
                    }
                    None => error!("Invalid character in file: {}", char),
                }
            } else {
                match char.to_digit(10) {
                    Some(k) => {
                        disk_str.push_str(&".".repeat(k as usize));
                    }
                    None => error!("Invalid character in free space: {}", char),
                }
            }
            is_file = !is_file;
        }

        debug!("Disk layout:\n{}", disk_str);
        let disk_layout: Vec<char> = disk_str.chars().collect();
        DiskMap { disk_layout }
    }

    fn get_first_free_space(&self) -> Option<usize> {
        self.disk_layout.iter().position(|&c| c == '.')
    }

    fn get_last_file(&self) -> Option<usize> {
        self.disk_layout.iter().rposition(|&c| c != '.')
    }

    fn move_file_to_free_space(&mut self, file_index: usize, free_space_index: usize) {
        self.disk_layout.swap(file_index, free_space_index);
    }

    fn defrag(&mut self) {
        info!(
            "Initial disk layout:\n{}",
            self.disk_layout.iter().collect::<String>()
        );

        while let Some(free_space_index) = self.get_first_free_space() {
            if let Some(last_file_index) = self.get_last_file() {
                if last_file_index < free_space_index {
                    break;
                }
                self.move_file_to_free_space(last_file_index, free_space_index);
            } else {
                break;
            };
        }

        info!(
            "Final disk layout:\n{}",
            self.disk_layout.iter().collect::<String>()
        );
    }

    fn calculate_checksum(&self) -> usize {
        self.disk_layout
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let f = c.to_digit(10).unwrap_or(0) as usize;
                debug!("{}, {} : {}", i, f, i * f);
                i * f
            })
            .sum::<usize>()
    }
}

fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let contents = fs::read_to_string("data/input")?;
    debug!("Loaded content:\n{}", contents);

    let mut disk_map = DiskMap::new(&contents);
    disk_map.defrag();
    info!("Checksum: {}", disk_map.calculate_checksum());

    Ok(())
}
