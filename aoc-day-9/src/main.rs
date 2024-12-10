use std::io::{self, Read};

#[derive(Debug, Clone)]
struct FileBlock {
    id: usize,
    size: usize,
}

#[derive(Debug)]
struct DiskState {
    blocks: Vec<Option<FileBlock>>,
}

impl DiskState {
    fn new(disk_map: &str) -> Self {
        let mut file_id = 0;
        let mut blocks = Vec::new();

        // Parse alternating file and space lengths
        for (i, c) in disk_map.chars().enumerate() {
            let length = c.to_digit(10).unwrap() as usize;

            if i % 2 == 0 {
                // File block
                for _ in 0..length {
                    blocks.push(Some(FileBlock {
                        id: file_id,
                        size: length,
                    }));
                }
                file_id += 1;
            } else {
                // Free space
                for _ in 0..length {
                    blocks.push(None);
                }
            }
        }

        DiskState { blocks }
    }

    // Part 1: Compact by moving individual blocks
    fn compact_blocks(&mut self) {
        let total_len = self.blocks.len();

        loop {
            let mut moved = false;

            // Find the rightmost file
            for pos in (0..total_len).rev() {
                if let Some(_file) = &self.blocks[pos] {
                    // Find leftmost free space
                    if let Some(target_pos) = self.blocks.iter().position(|block| block.is_none()) {
                        if target_pos < pos {
                            // Move the file block
                            let file_block = self.blocks[pos].take().unwrap();
                            self.blocks[target_pos] = Some(file_block);
                            moved = true;
                            break;
                        }
                    }
                    break;
                }
            }

            if !moved {
                break;
            }
        }
    }

    // Part 2: Compact by moving whole files
    fn compact_files(&mut self) {
        let max_id = self
            .blocks
            .iter()
            .filter_map(|block| block.as_ref().map(|file| file.id))
            .max()
            .unwrap_or(0);

        // Process files in order of decreasing ID
        for file_id in (0..=max_id).rev() {
            // Find the first block of the current file
            if let Some(start_pos) = self
                .blocks
                .iter()
                .position(|block| block.as_ref().map_or(false, |file| file.id == file_id))
            {
                // Get file size
                let file_size = self.blocks[start_pos].as_ref().unwrap().size;

                // Find leftmost viable free space
                let mut current_free = 0;
                let mut free_start = None;

                for (pos, block) in self.blocks.iter().enumerate() {
                    if pos >= start_pos {
                        break;
                    }

                    if block.is_none() {
                        if current_free == 0 {
                            free_start = Some(pos);
                        }
                        current_free += 1;

                        if current_free >= file_size {
                            // Found enough space, move the file
                            let target_pos = free_start.unwrap();

                            // Clear the old file blocks
                            for i in start_pos..start_pos + file_size {
                                self.blocks[i] = None;
                            }

                            // Place the file in its new position
                            for i in target_pos..target_pos + file_size {
                                self.blocks[i] = Some(FileBlock {
                                    id: file_id,
                                    size: file_size,
                                });
                            }
                            break;
                        }
                    } else {
                        current_free = 0;
                        free_start = None;
                    }
                }
            }
        }
    }

    fn calculate_checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(pos, block)| block.as_ref().map(|file| pos * file.id))
            .sum()
    }

    #[cfg(test)]
    fn to_debug_string(&self) -> String {
        self.blocks
            .iter()
            .map(|block| match block {
                None => '.'.to_string(),
                Some(file) => file.id.to_string(),
            })
            .collect()
    }
}

fn solve_part1(input: &str) -> usize {
    let mut disk = DiskState::new(input.trim());
    disk.compact_blocks();
    disk.calculate_checksum()
}

fn solve_part2(input: &str) -> usize {
    let mut disk = DiskState::new(input.trim());
    disk.compact_files();
    disk.calculate_checksum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let part1_result = solve_part1(&input);
    let part2_result = solve_part2(&input);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_state() {
        let disk = DiskState::new("2333133121414131402");
        assert_eq!(
            disk.to_debug_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_simple_example_part1() {
        let mut disk = DiskState::new("12345");
        assert_eq!(disk.to_debug_string(), "0..111....22222");

        disk.compact_blocks();
        assert_eq!(disk.to_debug_string(), "022111222......");
    }

    #[test]
    fn test_full_example_part1() {
        let input = "2333133121414131402";
        let result = solve_part1(input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_full_example_part2() {
        let input = "2333133121414131402";
        let mut disk = DiskState::new(input);
        assert_eq!(
            disk.to_debug_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );

        disk.compact_files();
        assert_eq!(
            disk.to_debug_string(),
            "00992111777.44.333....5555.6666.....8888.."
        );

        let result = solve_part2(input);
        assert_eq!(result, 2858);
    }
}
