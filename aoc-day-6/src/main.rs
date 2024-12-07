use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::io::{self, BufRead};

/// 📊 Analysis results from observing the guard's patrol patterns
#[derive(Debug)]
struct PatrolAnalysis {
    patrol_coverage: usize, // Number of unique positions visited by the guard
    paradox_points: usize,  // Number of positions that could create time loops
}

impl fmt::Display for PatrolAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let redact = f.sign_minus();

        let coverage = if redact {
            "*****".to_string()
        } else {
            format!("{} unique temporal coordinates", self.patrol_coverage)
        };

        let paradox = if redact {
            "*****".to_string()
        } else {
            format!("{} coordinates", self.paradox_points)
        };

        write!(
            f,
            "\n🕰️ Temporal Analysis Complete!\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            👮‍♂️ Guard's Patrol Coverage: {}\n\n\
            🌀 Paradox Analysis Complete!\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            ⚡️ Viable Paradox Points: {}\n\n\
            🏷️ Time Stream Analysis Concluded\n\n",
            coverage, paradox
        )
    }
}

// 🕰️ Using compact temporal coordinates to minimize butterfly effects in the timestream
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TemporalCoordinate {
    timeline_y: u8, // Using u8 saves temporal energy and improves quantum cache efficiency
    timeline_x: u8,
}

impl TemporalCoordinate {
    #[inline(always)]
    fn new(y: u8, x: u8) -> TemporalCoordinate {
        TemporalCoordinate {
            timeline_y: y,
            timeline_x: x,
        }
    }
}

// 🧭 Guard patrol vectors through spacetime
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum PatrolVector {
    Northward = 0, // Up relative to the temporal map
    Eastward = 1,  // Right in the timestream
    Southward = 2, // Down the timeline
    Westward = 3,  // Left through history
}

// ⚡️ Quantum lookup tables for instantaneous patrol calculations
const TEMPORAL_SHIFTS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const PROTOCOL_ROTATION: [PatrolVector; 4] = [
    PatrolVector::Eastward,
    PatrolVector::Southward,
    PatrolVector::Westward,
    PatrolVector::Northward,
];

impl PatrolVector {
    #[inline(always)]
    fn follow_protocol(self) -> PatrolVector {
        // Guards of 1518 always turn right when blocked
        PROTOCOL_ROTATION[self as usize]
    }

    #[inline(always)]
    fn calculate_movement(self) -> (i8, i8) {
        TEMPORAL_SHIFTS[self as usize]
    }
}

// 📜 The prototype suit manufacturing lab of 1518, encoded in quantum bits
struct TemporalLab {
    quantum_state: Vec<u8>, // Packed bits: 1 quantum bit per cell
    time_height: u8,
    time_width: u8,
    safe_coordinates: Vec<TemporalCoordinate>, // Positions that won't cause paradoxes
}

impl TemporalLab {
    fn new(height: u8, width: u8) -> TemporalLab {
        let size = ((height as usize * width as usize) + 7) / 8;
        TemporalLab {
            quantum_state: vec![0; size],
            time_height: height,
            time_width: width,
            safe_coordinates: Vec::new(),
        }
    }

    #[inline(always)]
    fn quantum_index(&self, pos: TemporalCoordinate) -> (usize, u8) {
        let index = pos.timeline_y as usize * self.time_width as usize + pos.timeline_x as usize;
        (index / 8, 1 << (index % 8))
    }

    #[inline(always)]
    fn exists_in_timeline(&self, pos: TemporalCoordinate) -> bool {
        pos.timeline_y < self.time_height && pos.timeline_x < self.time_width
    }

    #[inline(always)]
    fn has_obstacle(&self, pos: TemporalCoordinate) -> bool {
        if !self.exists_in_timeline(pos) {
            return false;
        }
        let (byte_idx, bit_mask) = self.quantum_index(pos);
        (self.quantum_state[byte_idx] & bit_mask) != 0
    }

    fn manifest_obstacle(&mut self, pos: TemporalCoordinate) {
        let (byte_idx, bit_mask) = self.quantum_index(pos);
        self.quantum_state[byte_idx] |= bit_mask;
    }

    fn map_safe_coordinates(&mut self, origin: TemporalCoordinate) {
        self.safe_coordinates = (0..self.time_height)
            .flat_map(|r| (0..self.time_width).map(move |c| TemporalCoordinate::new(r, c)))
            .filter(|&pos| pos != origin && !self.has_obstacle(pos))
            .collect();
    }
}

// 🕊️ A moment frozen in the timeline
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TimeSlice {
    location: TemporalCoordinate,
    heading: PatrolVector,
}

// ⚛️ Quantum-efficient tracking of guard movements
struct ParadoxTracker {
    quantum_memory: Vec<u64>,
}

impl ParadoxTracker {
    fn new(lab_height: u8, lab_width: u8) -> Self {
        let size = (lab_height as usize * lab_width as usize * 4 + 63) / 64;
        Self {
            quantum_memory: vec![0; size],
        }
    }

    #[inline(always)]
    fn calculate_index(state: TimeSlice, width: u8) -> usize {
        (state.location.timeline_y as usize * width as usize + state.location.timeline_x as usize)
            * 4
            + state.heading as usize
    }

    #[inline(always)]
    fn contains(&self, state: TimeSlice, width: u8) -> bool {
        let idx = Self::calculate_index(state, width);
        let (word_idx, bit_idx) = (idx / 64, idx % 64);
        (self.quantum_memory[word_idx] & (1 << bit_idx)) != 0
    }

    #[inline(always)]
    fn record(&mut self, state: TimeSlice, width: u8) {
        let idx = Self::calculate_index(state, width);
        let (word_idx, bit_idx) = (idx / 64, idx % 64);
        self.quantum_memory[word_idx] |= 1 << bit_idx;
    }

    fn reset_timeline(&mut self) {
        self.quantum_memory.fill(0);
    }
}

fn read_temporal_map() -> (TemporalLab, TemporalCoordinate, PatrolVector) {
    let stdin = io::stdin();
    let timeline: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.expect("🚫 Timeline corruption detected!"))
        .collect();

    let height = timeline.len() as u8;
    let width = timeline[0].len() as u8;
    let mut lab = TemporalLab::new(height, width);
    let mut guard_origin = None;
    let mut initial_vector = PatrolVector::Northward;

    // 🔍 Scan the temporal map for key elements
    for (y, line) in timeline.iter().enumerate() {
        for (x, quantum_state) in line.chars().enumerate() {
            let pos = TemporalCoordinate::new(y as u8, x as u8);
            match quantum_state {
                '#' => lab.manifest_obstacle(pos),
                '^' => {
                    guard_origin = Some(pos);
                    initial_vector = PatrolVector::Northward;
                }
                '.' => {}
                _ => panic!("⚠️ Temporal anomaly detected in map data!"),
            }
        }
    }

    let origin = guard_origin.expect("❌ Failed to locate guard's origin point in timeline!");
    lab.map_safe_coordinates(origin);
    (lab, origin, initial_vector)
}

fn trace_patrol_route(
    lab: &TemporalLab,
    origin: TemporalCoordinate,
    initial_vector: PatrolVector,
) -> HashSet<TemporalCoordinate> {
    let mut time_points = HashSet::new();
    let mut current_pos = origin;
    let mut current_vector = initial_vector;
    time_points.insert(current_pos);

    loop {
        let (dy, dx) = current_vector.calculate_movement();
        let next_pos = TemporalCoordinate::new(
            current_pos.timeline_y.checked_add_signed(dy).unwrap_or(255),
            current_pos.timeline_x.checked_add_signed(dx).unwrap_or(255),
        );

        if !lab.exists_in_timeline(next_pos) {
            break;
        }

        if lab.has_obstacle(next_pos) {
            current_vector = current_vector.follow_protocol();
        } else {
            current_pos = next_pos;
            time_points.insert(current_pos);
        }
    }
    time_points
}

fn simulate_timeline_with_obstacle(
    lab: &TemporalLab,
    origin: TemporalCoordinate,
    initial_vector: PatrolVector,
    paradox_point: TemporalCoordinate,
    tracker: &mut ParadoxTracker,
) -> bool {
    if !lab.exists_in_timeline(paradox_point)
        || lab.has_obstacle(paradox_point)
        || paradox_point == origin
    {
        return false;
    }

    tracker.reset_timeline();
    let mut current_state = TimeSlice {
        location: origin,
        heading: initial_vector,
    };

    loop {
        if tracker.contains(current_state, lab.time_width) {
            return true; // 🌀 Time loop detected!
        }
        tracker.record(current_state, lab.time_width);

        let (dy, dx) = current_state.heading.calculate_movement();
        let next_pos = TemporalCoordinate::new(
            current_state
                .location
                .timeline_y
                .checked_add_signed(dy)
                .unwrap_or(255),
            current_state
                .location
                .timeline_x
                .checked_add_signed(dx)
                .unwrap_or(255),
        );

        if !lab.exists_in_timeline(next_pos) {
            return false;
        }

        if lab.has_obstacle(next_pos) || next_pos == paradox_point {
            current_state.heading = current_state.heading.follow_protocol();
        } else {
            current_state.location = next_pos;
        }
    }
}

fn calculate_paradox_points(
    lab: &TemporalLab,
    origin: TemporalCoordinate,
    initial_vector: PatrolVector,
) -> usize {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    lab.safe_coordinates
        .par_chunks(256)
        .map(|chunk| {
            let mut tracker = ParadoxTracker::new(lab.time_height, lab.time_width);
            chunk
                .iter()
                .filter(|&&pos| {
                    simulate_timeline_with_obstacle(lab, origin, initial_vector, pos, &mut tracker)
                })
                .count()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let redact = std::env::args().any(|arg| arg == "--redact");

    let (lab, guard_origin, initial_vector) = read_temporal_map();

    // Analyze the temporal data
    let visited_points = trace_patrol_route(&lab, guard_origin, initial_vector);
    let paradox_count = calculate_paradox_points(&lab, guard_origin, initial_vector);

    // Prepare the analysis report
    let analysis = PatrolAnalysis {
        patrol_coverage: visited_points.len(),
        paradox_points: paradox_count,
    };

    // Display the results with optional redaction
    if redact {
        print!("{:-}", analysis); // The minus flag triggers redaction
    } else {
        print!("{}", analysis);
    }

    Ok(())
}

#[cfg(test)]
mod temporal_verification {
    use super::*;

    static HISTORICAL_RECORD: &str = "....#.....\n\
.........#\n\
..........\n\
..#.......\n\
.......#..\n\
..........\n\
.#..^.....\n\
........#.\n\
#.........\n\
......#...\n";

    #[test]
    fn verify_temporal_calculations() {
        let timeline: Vec<&str> = HISTORICAL_RECORD.lines().collect();
        let height = timeline.len() as u8;
        let width = timeline[0].len() as u8;
        let mut lab = TemporalLab::new(height, width);
        let mut origin = None;
        let mut vector = PatrolVector::Northward;

        for (y, line) in timeline.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = TemporalCoordinate::new(y as u8, x as u8);
                match ch {
                    '#' => lab.manifest_obstacle(pos),
                    '^' => {
                        origin = Some(pos);
                        vector = PatrolVector::Northward;
                    }
                    '.' => {}
                    _ => panic!("⚠️ Invalid historical record!"),
                }
            }
        }

        let origin_point = origin.unwrap();
        lab.map_safe_coordinates(origin_point);

        let visited = trace_patrol_route(&lab, origin_point, vector);
        assert_eq!(visited.len(), 41, "🚫 Patrol route calculation mismatch!");

        let loops = calculate_paradox_points(&lab, origin_point, vector);
        assert_eq!(loops, 6, "🚫 Paradox point calculation mismatch!");
    }
}
