use std::{cmp, collections::HashSet};

type Grid = Vec<Vec<u32>>;
type Coord = (usize, usize);

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn trailheads(map: &Grid) -> Vec<Coord> {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, value)| match value {
                    0 => Some((y, x)),
                    _ => None,
                })
        })
        .collect()
}

fn neighbours(coord: &Coord, max_x: usize, max_y: usize) -> HashSet<Coord> {
    HashSet::from([
        (coord.0.checked_sub(1).unwrap_or_default(), coord.1),
        (coord.0, cmp::min(coord.1 + 1, max_x)),
        (cmp::min(coord.0 + 1, max_y), coord.1),
        (coord.0, coord.1.checked_sub(1).unwrap_or_default()),
    ])
}

fn one_higher_neighbors(coord: &Coord, map: &Grid) -> Vec<Coord> {
    neighbours(coord, map[0].len() - 1, map.len() - 1)
        .into_iter()
        .filter(|(y, x)| {
            map[*y][*x]
                .checked_sub(map[coord.0][coord.1])
                .is_some_and(|diff| diff == 1)
        })
        .collect()
}

fn find_peaks(trailhead: &Coord, map: &Grid) -> Vec<Coord> {
    match map[trailhead.0][trailhead.1] {
        9 => vec![*trailhead],
        _ => one_higher_neighbors(&trailhead, map)
            .iter()
            .flat_map(|neighbour| find_peaks(neighbour, map))
            .collect(),
    }
}

fn trail_scores(map: &Grid, paths: bool) -> usize {
    let unique_peaks = |h, m| find_peaks(&h, m).iter().collect::<HashSet<_>>().len();
    let unique_paths = |h, m| find_peaks(&h, m).len();

    trailheads(map)
        .into_iter()
        .map(|trailhead| match paths {
            true => unique_paths(trailhead, map),
            false => unique_peaks(trailhead, map),
        })
        .sum()
}

fn main() {
    let map = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", trail_scores(&map, false));
    println!("Part 2: {}", trail_scores(&map, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(trail_scores(&parse_input(include_str!("../input-test.txt")), false) == 36);
    }

    #[test]
    fn example_2() {
        assert!(trail_scores(&parse_input(include_str!("../input-test.txt")), true) == 81);
    }
}
