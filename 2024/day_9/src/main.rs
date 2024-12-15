use std::collections::BTreeMap;
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
    contiguous_files: BTreeMap<usize, (usize, usize)>,
    contiguous_free_space: BTreeMap<usize, usize>,
}

impl DiskMap {
    fn new(contents: &str) -> Self {
        let mut disk_layout = Vec::new();
        let mut contiguous_files = BTreeMap::new();
        let mut contiguous_free_space = BTreeMap::new();

        let mut is_file = true;
        let mut file_id = 0;
        let mut index = 0;
        for char in contents.chars() {
            match char.to_digit(10) {
                Some(count) => {
                    let count = count as usize;
                    if is_file {
                        disk_layout.extend(
                            std::iter::repeat(DiskContent::File { id: file_id }).take(count),
                        );
                        contiguous_files.insert(file_id, (index, count));
                        file_id += 1;
                    } else {
                        disk_layout.extend(std::iter::repeat(DiskContent::FreeSpace).take(count));
                        contiguous_free_space.insert(index, count);
                    }
                    is_file = !is_file;
                    index += count;
                }
                None => {
                    error!("Invalid character: {}", char);
                }
            }
        }

        DiskMap {
            disk_layout,
            contiguous_files,
            contiguous_free_space,
        }
    }
    fn print_layout(&self) -> String {
        self.disk_layout
            .iter()
            .map(|content| format!("| {} ", content))
            .collect::<String>()
            + "|"
    }

    fn print_layout_contiguous(&self) -> String {
        let max_pos = self
            .contiguous_files
            .values()
            .map(|(pos, len)| pos + len)
            .max()
            .unwrap_or(0);

        let mut result = String::with_capacity(max_pos);
        let mut current_pos = 0;

        while current_pos < max_pos {
            // Check if there's a file at current position
            if let Some((&file_id, &(_start_pos, len))) = self
                .contiguous_files
                .iter()
                .find(|(_, &(pos, _))| pos == current_pos)
            {
                result.push_str(&format!("[{}]", file_id).repeat(len));
                current_pos += len;
            }
            // Check if there's free space at current position
            else if let Some(&len) = self.contiguous_free_space.get(&current_pos) {
                result.push_str(&".".repeat(len));
                current_pos += len;
            }
            // If neither found, move to next position
            else {
                current_pos += 1;
            }
        }
        result
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
    fn defrag_contiguous(&mut self) {
        // Iterate through files in reverse order of position
        let files_to_process: Vec<(usize, usize, usize)> = self
            .contiguous_files
            .iter()
            .rev()
            .map(|(&id, &(pos, len))| (id, pos, len))
            .collect();

        for (file_id, file_pos, file_len) in files_to_process {
            // Find suitable free space
            debug!("Processing file {} at {}", file_id, file_pos);
            if let Some((&free_start, _free_len)) = self
                .contiguous_free_space
                .range(..file_pos)
                .find(|(_, &len)| len >= file_len)
            {
                // Move file to new location
                debug!(
                    "Moving file {} from {} to {}",
                    file_id, file_pos, free_start
                );
                self.move_file(file_id, file_pos, file_len, free_start);
            }
        }
    }

    fn move_file(&mut self, file_id: usize, old_pos: usize, file_len: usize, new_pos: usize) {
        // Remove or shrink free space
        if let Some(free_len) = self.contiguous_free_space.remove(&new_pos) {
            self.contiguous_free_space.remove(&new_pos);
            if free_len > file_len {
                self.contiguous_free_space
                    .insert(new_pos + file_len, free_len - file_len);
            }
        }

        // Move file to new pos
        self.contiguous_files.insert(file_id, (new_pos, file_len));

        // Add free space to old file pos
        self.add_free_space(old_pos, file_len);
    }

    fn add_free_space(&mut self, pos: usize, len: usize) {
        let mut merged_start = pos;
        let mut merged_len = len;

        // Check previous free space to see if we need to merge
        if let Some((&prev_start, prev_len)) = self
            .contiguous_free_space
            .range(..pos)
            .rev()
            .next()
            .filter(|(&start, &len)| start + len == pos)
        {
            merged_start = prev_start;
            merged_len += prev_len;
            self.contiguous_free_space.remove(&prev_start);
        }

        // Same for next free space
        if let Some(next_len) = self.contiguous_free_space.remove(&(pos + len)) {
            merged_len += next_len;
        }

        // Insert merged free space
        self.contiguous_free_space.insert(merged_start, merged_len);
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

    fn calculate_checksum_contiguous(&self) -> usize {
        self.contiguous_files
            .iter()
            .flat_map(|(&id, &(index, len))| (index..index + len).map(move |i| i * id))
            .sum::<usize>()
    }
}

fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let contents = fs::read_to_string("data/input")?.trim().to_string();
    debug!("Loaded content:\n{}", contents);

    let mut disk_map = DiskMap::new(&contents);

    // Part 1
    info!("Part 1");
    debug!("Initial layout:\n{}", disk_map.print_layout());
    disk_map.defrag();
    debug!("Defragged layout:\n{}", disk_map.print_layout());
    info!("Checksum: {}", disk_map.calculate_checksum());

    // Part 2
    info!("\n\n\nPart 2");
    debug!("Initial layout:\n{}", disk_map.print_layout_contiguous());
    disk_map.defrag_contiguous();
    debug!("Defragged layout:\n{}", disk_map.print_layout_contiguous());
    info!("Checksum: {}", disk_map.calculate_checksum_contiguous());

    Ok(())
}
