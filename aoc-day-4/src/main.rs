//! Ceres Search Adventure - Main Module
//!
//! "The Historians need to continue their search for the elusive Chief, but a small Elf on the Ceres station has a puzzle."
//!
//! After helping the Elf find "XMAS" hidden across various directions, the real challenge emerges. The "X-MAS" puzzle requires finding a more intricate cross pattern involving two "MAS" words.
//! This file serves as the entry point to launch the Elf's adventure and solve the word search.

use aoc_day_4::Grid;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("⚠️ Oh dear! It seems you've forgotten to provide the input file. Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1])?;
    let grid: Grid = input.parse()?;

    // The Elf watches intently as you search for the hidden XMAS words in her puzzle.
    let word_count = grid.find_word_xmas();
    println!("\n✨ Magical Word Analysis: '\x1b[1;34mXMAS\x1b[0m' Found!");
    println!("🔍 Total 'XMAS' Instances Found: {}", word_count);

    println!("{}", "-".repeat(50));

    // The Elf is even more eager to see if you can find the tricky X-MAS cross patterns.
    let cross_count = grid.find_crossed_mas();
    println!("🧩 Cross Pattern Analysis: Searching for 'X-MAS' Crosses...");
    println!("📌 Total 'X-MAS' Cross Patterns Found: {}\n", cross_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_narrative() {
        let input = "..X...\n\
                    .SAMX.\n\
                    .A..A.\n\
                    XMAS.S\n\
                    .X....";
        let grid: Grid = input.parse().unwrap();
        println!("Input grid:\n{}", grid);
        let count = grid.find_word_xmas();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_actual_puzzle() {
        let input = "MMMSXXMASM\n\
                    MSAMXMSMSA\n\
                    AMXSXMAAMM\n\
                    MSAMASMSMX\n\
                    XMASAMXAMM\n\
                    XXAMMXXAMA\n\
                    SMSMSASXSS\n\
                    SAXAMASAAA\n\
                    MAMMMXMMMM\n\
                    MXMXAXMASX";
        let grid: Grid = input.parse().unwrap();
        let count = grid.find_word_xmas();
        assert_eq!(count, 18);
    }

    #[test]
    fn test_crossed_mas_full_example() {
        let input = ".M.S......\n\
                    ..A..MSMS.\n\
                    .M.S.MAA..\n\
                    ..A.ASMSM.\n\
                    .M.S.M....\n\
                    ..........\n\
                    S.S.S.S.S.\n\
                    .A.A.A.A..\n\
                    M.M.M.M.M.\n\
                    ..........";
        let grid: Grid = input.parse().unwrap();
        println!("Input grid:\n{}", grid);
        let count = grid.find_crossed_mas();
        assert_eq!(count, 9);
    }

    #[test]
    fn test_crossed_mas_simple() {
        let input = "M.S\n\
                    .A.\n\
                    M.S";
        let grid: Grid = input.parse().unwrap();
        println!("Input grid:\n{}", grid);
        let count = grid.find_crossed_mas();
        assert_eq!(count, 1);
    }
}
