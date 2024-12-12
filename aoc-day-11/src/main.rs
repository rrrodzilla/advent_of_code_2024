use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let mut stone_memory = HashMap::new(); // Ancient cache of previously observed stone states
    for (blinks, phase) in [(25, "Phase 1"), (75, "Phase 2")] {
        println!(
            "\n✨ 🌌 Cosmic Phase {} (after {} blinks): {}",
            phase,
            blinks,
            stones
                .iter()
                .map(|stone| observe_stone_transformations(stone, blinks, &mut stone_memory))
                .sum::<usize>()
        );
        stone_memory.clear();
    }
    Ok(())
}

fn observe_stone_transformations(
    stone: &str,
    blinks: usize,
    stone_memory: &mut HashMap<(String, usize), usize>,
) -> usize {
    if let Some(&result) = stone_memory.get(&(stone.to_string(), blinks)) {
        return result; // Return previously observed transformation
    }
    let result = match (blinks, stone) {
        (0, _) => 1, // Stone remains unchanged
        (n, "0") => observe_stone_transformations("1", n - 1, stone_memory), // Null stone transforms to unity
        (n, s) if has_balanced_inscriptions(s) => {
            let (left, right) = s.split_at(s.len() / 2);
            observe_stone_transformations(clean_inscription(left), n - 1, stone_memory)
                + observe_stone_transformations(clean_inscription(right), n - 1, stone_memory)
        }
        (n, s) => observe_stone_transformations(&apply_plutonian_constant(s), n - 1, stone_memory), // Apply cosmic multiplication
    };
    stone_memory.insert((stone.to_string(), blinks), result);
    result
}

fn has_balanced_inscriptions(s: &str) -> bool {
    let len = clean_inscription(s).len();
    len > 0 && len % 2 == 0
}

fn apply_plutonian_constant(s: &str) -> String {
    let mut result = Vec::new();
    let mut cosmic_overflow = 0u64;
    for d in s.chars().rev().map(|c| c.to_digit(10).unwrap() as u64) {
        let cosmic_value = d * 2024 + cosmic_overflow;
        result.push((cosmic_value % 10) as u8);
        cosmic_overflow = cosmic_value / 10;
    }
    while cosmic_overflow > 0 {
        result.push((cosmic_overflow % 10) as u8);
        cosmic_overflow /= 10;
    }
    result.iter().rev().map(|&d| (d + b'0') as char).collect()
}

fn clean_inscription(s: &str) -> &str {
    let s = s.trim_start_matches('0');
    if s.is_empty() {
        "0"
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_sequence() {
        let mut stone_memory = HashMap::new();
        assert_eq!(
            ["125", "17"]
                .iter()
                .map(|s| observe_stone_transformations(s, 3, &mut stone_memory))
                .sum::<usize>(),
            5
        );
    }
    #[test]
    fn test_single_blink_example() {
        let mut stone_memory = HashMap::new();
        assert_eq!(
            ["0", "1", "10", "99", "999"]
                .iter()
                .map(|s| observe_stone_transformations(s, 1, &mut stone_memory))
                .sum::<usize>(),
            7
        );
    }
}
