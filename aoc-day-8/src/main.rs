use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::io::{self, BufRead};

/// 📻 Represents a signal frequency (digit, uppercase, or lowercase character)
type Frequency = char;

/// 🎛️ Calculates the greatest common divisor for signal phase alignment
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// 📡 SignalMap: Core system for analyzing antenna resonance patterns
#[derive(Debug)]
struct SignalMap {
    // Maps frequencies to arrays of antenna positions
    frequency_network: HashMap<Frequency, Vec<(i32, i32)>>,
    // Transmission field dimensions
    field_width: usize,
    field_height: usize,
}

impl From<&[String]> for SignalMap {
    fn from(transmission_data: &[String]) -> Self {
        let field_height = transmission_data.len();
        let field_width = if field_height > 0 {
            transmission_data[0].len()
        } else {
            0
        };

        // 📶 Build frequency-to-position mapping
        let frequency_network: HashMap<Frequency, Vec<(i32, i32)>> = transmission_data
            .iter()
            .enumerate()
            .flat_map(|(y, scan_line)| {
                scan_line
                    .chars()
                    .enumerate()
                    .filter(|&(_, signal)| signal.is_alphanumeric())
                    .map(move |(x, signal)| (signal, (x as i32, y as i32)))
            })
            .fold(HashMap::new(), |mut network, (signal, pos)| {
                network.entry(signal).or_default().push(pos);
                network
            });

        Self {
            frequency_network,
            field_width,
            field_height,
        }
    }
}

/// 🔍 Core trait for different antinode detection algorithms
trait ResonanceDetector {
    fn detect_antinodes(
        &self,
        antennas: &[(i32, i32)],
        field_width: usize,
        field_height: usize,
    ) -> HashSet<(i32, i32)>;
}

/// 📊 Phase One: Distance-based resonance detection
struct DistanceResonance;

impl ResonanceDetector for DistanceResonance {
    fn detect_antinodes(
        &self,
        antennas: &[(i32, i32)],
        field_width: usize,
        field_height: usize,
    ) -> HashSet<(i32, i32)> {
        let mut resonance_points = HashSet::new();

        // 📏 Analyze each antenna pair for distance-based resonance
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (ax, ay) = antennas[i];
                let (bx, by) = antennas[j];

                // 🎯 Calculate resonance points at double distance intervals
                let forward_x = 2 * ax - bx;
                let forward_y = 2 * ay - by;
                if forward_x >= 0
                    && forward_x < field_width as i32
                    && forward_y >= 0
                    && forward_y < field_height as i32
                {
                    resonance_points.insert((forward_x, forward_y));
                }

                let backward_x = 2 * bx - ax;
                let backward_y = 2 * by - ay;
                if backward_x >= 0
                    && backward_x < field_width as i32
                    && backward_y >= 0
                    && backward_y < field_height as i32
                {
                    resonance_points.insert((backward_x, backward_y));
                }
            }
        }
        resonance_points
    }
}

/// 🌌 Phase Two: Harmonic resonance pattern detection
struct HarmonicResonance;

impl ResonanceDetector for HarmonicResonance {
    fn detect_antinodes(
        &self,
        antennas: &[(i32, i32)],
        field_width: usize,
        field_height: usize,
    ) -> HashSet<(i32, i32)> {
        let mut resonance_points = HashSet::new();

        // 🎵 Analyze harmonic patterns between antenna pairs
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (ax, ay) = antennas[i];
                let (bx, by) = antennas[j];

                // 📐 Calculate harmonic phase alignment
                let delta_x = bx - ax;
                let delta_y = by - ay;
                let phase = gcd(delta_x, delta_y);
                let step_x = delta_x / phase;
                let step_y = delta_y / phase;

                // 🌊 Trace forward wave propagation
                let (mut fx, mut fy) = (ax, ay);
                while fx >= 0 && fx < field_width as i32 && fy >= 0 && fy < field_height as i32 {
                    resonance_points.insert((fx, fy));
                    fx += step_x;
                    fy += step_y;
                }

                // 🌊 Trace backward wave propagation
                let (mut bx, mut by) = (ax, ay);
                while bx >= 0 && bx < field_width as i32 && by >= 0 && by < field_height as i32 {
                    resonance_points.insert((bx, by));
                    bx -= step_x;
                    by -= step_y;
                }
            }
        }
        resonance_points
    }
}

/// 🎯 Analyze total resonance pattern across the signal map
fn analyze_resonance<D: ResonanceDetector>(signal_map: &SignalMap, detector: D) -> usize {
    signal_map
        .frequency_network
        .values()
        .flat_map(|antennas| {
            detector.detect_antinodes(antennas, signal_map.field_width, signal_map.field_height)
        })
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    println!("📡 Initializing antenna resonance analysis...");

    // 📥 Load transmission data
    let stdin = io::stdin();
    let scan_data: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("📶 Processing signal map...");
    let signal_map: SignalMap = scan_data.as_slice().into();

    // 📊 Phase One: Distance-based resonance analysis
    println!("\n🔍 Phase One: Analyzing distance-based resonance patterns");
    let phase_one_result = analyze_resonance(&signal_map, DistanceResonance);
    println!(
        "📈 Identified {} primary resonance points",
        phase_one_result
    );

    // 🌌 Phase Two: Harmonic resonance analysis
    println!("\n🎵 Phase Two: Analyzing harmonic frequency patterns");
    let phase_two_result = analyze_resonance(&signal_map, HarmonicResonance);
    println!(
        "📈 Identified {} harmonic resonance points",
        phase_two_result
    );

    println!("\n✨ Signal analysis complete!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_resonance_pattern() {
        // 🧪 Test case for primary resonance detection
        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let signal_map: SignalMap = input.as_slice().into();
        let resonance_count = analyze_resonance(&signal_map, DistanceResonance);
        assert_eq!(
            resonance_count, 14,
            "🚨 Expected 14 primary resonance points"
        );
    }

    #[test]
    fn test_harmonic_resonance_t_frequency() {
        // 🧪 Test case for T-frequency harmonic pattern
        let input = [
            "T....#....",
            "...T......",
            ".T....#...",
            ".........#",
            "..#.......",
            "..........",
            "...#......",
            "..........",
            "....#.....",
            "..........",
        ];
        let clean_input: Vec<String> = input.iter().map(|l| l.replace("#", ".")).collect();
        let signal_map: SignalMap = clean_input.as_slice().into();
        let resonance_count = analyze_resonance(&signal_map, HarmonicResonance);
        assert_eq!(
            resonance_count, 9,
            "🚨 Expected 9 harmonic resonance points"
        );
    }

    #[test]
    fn test_full_harmonic_pattern() {
        // 🧪 Test case for complete harmonic analysis
        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let signal_map: SignalMap = input.as_slice().into();
        let resonance_count = analyze_resonance(&signal_map, HarmonicResonance);
        assert_eq!(
            resonance_count, 34,
            "🚨 Expected 34 total harmonic resonance points"
        );
    }
}
