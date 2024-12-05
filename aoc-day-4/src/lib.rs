use std::fmt;
use std::str::FromStr;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Debug)]
pub struct ParseGridError;

impl fmt::Display for ParseGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse grid - the Elf seems confused, please check the format!"
        )
    }
}

impl std::error::Error for ParseGridError {}

#[derive(Debug, Clone, Copy)]
struct Direction {
    dy: i8,
    dx: i8,
}

impl Direction {
    /// All possible directions to search for the word "XMAS".
    /// The Elf has drawn small arrows in eight directions, making sure you know where to look.
    const ALL: [Direction; 8] = [
        Direction { dy: 0, dx: 1 },   // Right
        Direction { dy: 1, dx: 0 },   // Down
        Direction { dy: 0, dx: -1 },  // Left
        Direction { dy: -1, dx: 0 },  // Up
        Direction { dy: 1, dx: 1 },   // DownRight
        Direction { dy: 1, dx: -1 },  // DownLeft
        Direction { dy: -1, dx: 1 },  // UpRight
        Direction { dy: -1, dx: -1 }, // UpLeft
    ];
}

#[derive(Debug)]
pub struct Grid {
    cells: Vec<u8>,
    height: usize,
    width: usize,
    x_positions: Vec<(usize, usize)>,
    #[cfg(target_arch = "x86_64")]
    row_offsets: Vec<usize>,
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or(ParseGridError)?.len();
        let mut cells = Vec::with_capacity(s.len());
        let mut x_positions = Vec::new();

        // The Elf watches closely as each row is processed, making sure no "X" is missed.
        for (row, line) in s.lines().enumerate() {
            if line.len() != width {
                return Err(ParseGridError);
            }

            for (col, &byte) in line.as_bytes().iter().enumerate() {
                if byte == b'X' {
                    x_positions.push((row, col));
                }
            }

            cells.extend_from_slice(line.as_bytes());
        }

        let height = cells.len() / width;

        #[cfg(target_arch = "x86_64")]
        let row_offsets: Vec<usize> = (0..height).map(|r| r * width).collect();

        Ok(Grid {
            cells,
            height,
            width,
            x_positions,
            #[cfg(target_arch = "x86_64")]
            row_offsets,
        })
    }
}

impl Grid {
    /// Retrieve the value at a specific position in the grid.
    /// The Elf peeks over your shoulder as you double-check each character.
    #[inline(always)]
    fn get(&self, row: usize, col: usize) -> u8 {
        debug_assert!(row < self.height && col < self.width);
        unsafe { *self.cells.get_unchecked(row * self.width + col) }
    }

    /// Find all occurrences of the word "XMAS" within the grid.
    /// The Elf's excitement grows as you uncover each "XMAS" hidden in the puzzle.
    pub fn find_word_xmas(&self) -> usize {
        let mut count = 0;

        // Starting the search for each 'X' found in the grid.
        for &(row, col) in &self.x_positions {
            let pos = row * self.width + col;

            for dir in Direction::ALL {
                // Calculating the endpoint for the direction we're exploring.
                let end_row = row as i32 + dir.dy as i32 * 3;
                let end_col = col as i32 + dir.dx as i32 * 3;

                // Ensure the endpoint is within the bounds of the grid.
                if end_row < 0
                    || end_row >= self.height as i32
                    || end_col < 0
                    || end_col >= self.width as i32
                {
                    continue;
                }

                // Calculate the position offsets for checking "MAS" following the 'X'.
                let base_offset = dir.dy as isize * self.width as isize + dir.dx as isize;

                unsafe {
                    let pos1 = (pos as isize + base_offset) as usize;
                    let pos2 = (pos1 as isize + base_offset) as usize;
                    let pos3 = (pos2 as isize + base_offset) as usize;

                    // The Elf cheers as you find another occurrence of "XMAS"!
                    if *self.cells.get_unchecked(pos1) == b'M'
                        && *self.cells.get_unchecked(pos2) == b'A'
                        && *self.cells.get_unchecked(pos3) == b'S'
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// SIMD-optimized search for the X-MAS cross pattern for x86_64 architectures.
    /// The Elf is fascinated by the speed with which this part of the search happens.
    #[cfg(target_arch = "x86_64")]
    #[inline(never)]
    #[target_feature(enable = "avx2")]
    unsafe fn find_crossed_mas_simd(&self) -> usize {
        let mut count = 0;
        let a_pattern = _mm256_set1_epi8(b'A' as i8);

        // Constants for pattern matching
        const MS_PATTERN: u16 = ((b'M' as u16) << 8) | (b'S' as u16);
        const SM_PATTERN: u16 = ((b'S' as u16) << 8) | (b'M' as u16);

        // Process 32 positions at once
        for row in 1..self.height - 1 {
            let row_offset = *self.row_offsets.get_unchecked(row);
            let above_offset = *self.row_offsets.get_unchecked(row - 1);
            let below_offset = *self.row_offsets.get_unchecked(row + 1);

            let mut col = 1;
            while col + 32 < self.width - 1 {
                let center = _mm256_loadu_si256(
                    self.cells.get_unchecked(row_offset + col) as *const u8 as *const __m256i
                );
                let a_matches = _mm256_cmpeq_epi8(center, a_pattern);
                let mask = _mm256_movemask_epi8(a_matches) as u32;

                if mask != 0 {
                    let mut bit = mask;
                    while bit != 0 {
                        let pos = bit.trailing_zeros() as usize;
                        let c = col + pos;

                        // Load diagonal characters to check for 'M' and 'S'
                        let ul = *self.cells.get_unchecked(above_offset + c - 1);
                        let ur = *self.cells.get_unchecked(above_offset + c + 1);
                        let ll = *self.cells.get_unchecked(below_offset + c - 1);
                        let lr = *self.cells.get_unchecked(below_offset + c + 1);

                        let ul_lr: u16 = ((ul as u16) << 8) | (lr as u16);
                        let ur_ll: u16 = ((ur as u16) << 8) | (ll as u16);

                        // If we find a valid 'MAS' cross pattern, the Elf claps with joy.
                        count += ((ul_lr == MS_PATTERN || ul_lr == SM_PATTERN)
                            && (ur_ll == MS_PATTERN || ur_ll == SM_PATTERN))
                            as usize;

                        bit &= bit - 1; // Clear least significant set bit
                    }
                }
                col += 32;
            }

            // Handle remaining positions
            while col < self.width - 1 {
                if *self.cells.get_unchecked(row_offset + col) == b'A' {
                    let ul = *self.cells.get_unchecked(above_offset + col - 1);
                    let ur = *self.cells.get_unchecked(above_offset + col + 1);
                    let ll = *self.cells.get_unchecked(below_offset + col - 1);
                    let lr = *self.cells.get_unchecked(below_offset + col + 1);

                    let ul_lr: u16 = ((ul as u16) << 8) | (lr as u16);
                    let ur_ll: u16 = ((ur as u16) << 8) | (ll as u16);

                    count += ((ul_lr == MS_PATTERN || ul_lr == SM_PATTERN)
                        && (ur_ll == MS_PATTERN || ur_ll == SM_PATTERN))
                        as usize;
                }
                col += 1;
            }
        }
        count
    }

    /// Find all occurrences of the "X-MAS" cross pattern within the grid.
    /// The Elf looks amazed every time you uncover another 'X-MAS' cross formation.
    pub fn find_crossed_mas(&self) -> usize {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            self.find_crossed_mas_simd()
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            let mut count = 0;
            for row in 1..self.height - 1 {
                let row_offset = row * self.width;
                for col in 1..self.width - 1 {
                    if self.cells[row_offset + col] == b'A' {
                        let ul = self.cells[row_offset - self.width + col - 1];
                        let ur = self.cells[row_offset - self.width + col + 1];
                        let ll = self.cells[row_offset + self.width + col - 1];
                        let lr = self.cells[row_offset + self.width + col + 1];

                        let ul_lr: u16 = ((ul as u16) << 8) | (lr as u16);
                        let ur_ll: u16 = ((ur as u16) << 8) | (ll as u16);

                        count += ((ul_lr == MS_PATTERN || ul_lr == SM_PATTERN)
                            && (ur_ll == MS_PATTERN || ur_ll == SM_PATTERN))
                            as usize;
                    }
                }
            }
            count
        }
    }

    /// Returns the dimensions of the grid.
    /// The Elf quickly notes down the height and width of her puzzle for future reference.
    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}

impl fmt::Display for Grid {
    /// Nicely format the grid for display purposes.
    /// The Elf smiles as you neatly display her puzzle on the screen.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.get(row, col) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
