use aoc_day_4::Grid;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1])?;
    let grid: Grid = input.parse()?;

    let word_count = grid.find_word_xmas();
    let cross_count = grid.find_crossed_mas();

    println!("Found {} instances of XMAS as a word", word_count);
    println!("Found {} instances of crossed MAS patterns", cross_count);

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
