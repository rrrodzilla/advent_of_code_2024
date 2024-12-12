use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let trailhead_scores = sum_of_trailhead_scores(&map);
    let trailhead_ratings = sum_of_trailhead_ratings(&map);
    println!("{} {}", trailhead_scores, trailhead_ratings);
}

fn sum_of_trailhead_scores(map: &[Vec<u8>]) -> usize {
    let mut total_score = 0;
    let rows = map.len();
    let cols = map[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                total_score += bfs_trailhead_score(map, r, c);
            }
        }
    }

    total_score
}

fn bfs_trailhead_score(map: &[Vec<u8>], start_r: usize, start_c: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1isize), (-1isize, 0)];
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut score = 0;

    queue.push_back((start_r, start_c, 0));
    visited[start_r][start_c] = true;

    while let Some((r, c, height)) = queue.pop_front() {
        if height == 9 {
            score += 1;
            continue;
        }

        for &(dr, dc) in &directions {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;

            if new_r >= 0
                && new_r < map.len() as isize
                && new_c >= 0
                && new_c < map[0].len() as isize
            {
                let new_r = new_r as usize;
                let new_c = new_c as usize;
                if !visited[new_r][new_c] && map[new_r][new_c] == height + 1 {
                    visited[new_r][new_c] = true;
                    queue.push_back((new_r, new_c, height + 1));
                }
            }
        }
    }

    score
}

fn sum_of_trailhead_ratings(map: &[Vec<u8>]) -> usize {
    let mut total_rating = 0;
    let rows = map.len();
    let cols = map[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                total_rating += trailhead_rating(map, r, c);
            }
        }
    }

    total_rating
}

fn trailhead_rating(map: &[Vec<u8>], start_r: usize, start_c: usize) -> usize {
    let mut memo = HashMap::new();
    count_paths(map, start_r, start_c, &mut memo)
}

fn count_paths(
    map: &[Vec<u8>],
    r: usize,
    c: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let h = map[r][c];
    if h == 9 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(r, c)) {
        return cached;
    }

    let directions = [(0, 1isize), (1isize, 0), (0, -1isize), (-1isize, 0)];
    let mut total_paths = 0;
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    for &(dr, dc) in &directions {
        let new_r = r as isize + dr;
        let new_c = c as isize + dc;

        if new_r >= 0 && new_r < rows && new_c >= 0 && new_c < cols {
            let nr = new_r as usize;
            let nc = new_c as usize;
            if map[nr][nc] == h + 1 {
                total_paths += count_paths(map, nr, nc, memo);
            }
        }
    }

    memo.insert((r, c), total_paths);
    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(sum_of_trailhead_scores(&map), 36);
    }

    #[test]
    fn test_example_part_two() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        assert_eq!(sum_of_trailhead_ratings(&map), 81);
    }
}
