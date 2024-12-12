use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Square {
    x: usize,
    y: usize,
    letter: char,
    region: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    regions: HashMap<(usize, usize), Square>,
    max_x: usize,
    max_y: usize,
}

fn main() {
    let map = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", cost(&map));
    println!("Part 2: {}", bulk_cost(&map));
}

fn neighbors(coord: (usize, usize), max_x: usize, max_y: usize) -> HashSet<(usize, usize)> {
    HashSet::from([
        (coord.0.checked_sub(1).unwrap_or_default(), coord.1),
        (coord.0, cmp::min(coord.1 + 1, max_y)),
        (cmp::min(coord.0 + 1, max_x), coord.1),
        (coord.0, coord.1.checked_sub(1).unwrap_or_default()),
    ])
}

fn neighbors_with_ob(coord: (usize, usize)) -> Vec<(Option<usize>, Option<usize>)> {
    vec![
        vertical_neighbors_with_ob(coord),
        horizontal_neighbors_with_ob(coord),
    ]
    .concat()
}

fn vertical_neighbors_with_ob(coord: (usize, usize)) -> Vec<(Option<usize>, Option<usize>)> {
    vec![
        (Some(coord.0), Some(coord.1 + 1)),
        (Some(coord.0), coord.1.checked_sub(1)),
    ]
}

fn horizontal_neighbors_with_ob(coord: (usize, usize)) -> Vec<(Option<usize>, Option<usize>)> {
    vec![
        (coord.0.checked_sub(1), Some(coord.1)),
        (Some(coord.0 + 1), Some(coord.1)),
    ]
}

fn get_region(coord: (usize, usize), map: &Vec<Vec<char>>, existing: &mut Vec<(usize, usize)>) {
    let region_char = map[coord.1][coord.0];
    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    let new_neighbors: Vec<_> = neighbors(coord, max_x, max_y)
        .into_iter()
        .filter(|n| map[n.1][n.0] == region_char)
        .filter(|n| !existing.contains(n))
        .collect();

    if new_neighbors.is_empty() {
        return;
    }

    existing.append(&mut new_neighbors.clone());
    for neighbor in new_neighbors {
        get_region(neighbor, map, existing);
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = HashMap::new();

    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();

    let mut region_id = 0;

    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if !map.contains_key(&(x, y)) {
                let mut squares = vec![(x, y)];
                get_region((x, y), &grid, &mut squares);
                for square in squares {
                    map.insert(square, Square {
                        x: square.0,
                        y: square.1,
                        letter: *c,
                        region: region_id,
                    });
                }
                region_id += 1;
            }
        })
    });

    Map {
        regions: map,
        max_x: grid.len() - 1,
        max_y: grid[0].len() - 1,
    }
}

fn areas(map: &Map) -> HashMap<usize, usize> {
    map.regions.values().counts_by(|s| s.region)
}

fn perimeters(map: &Map) -> HashMap<usize, usize> {
    let regions = map.regions.values().into_group_map_by(|s| s.region);

    regions
        .iter()
        .map(|(id, squares)| {
            let region_coords = squares.iter().map(|s| (s.x, s.y)).collect_vec();
            (
                *id,
                squares
                    .iter()
                    .flat_map(|s| neighbors_with_ob((s.x, s.y)))
                    .filter(|n| match (n.0, n.1) {
                        (Some(x), Some(y)) => !region_coords.contains(&(x, y)),
                        _ => true,
                    })
                    .count(),
            )
        })
        .collect()
}

fn count_continuous_streaks(coords: Vec<Option<usize>>) -> usize {
    let num_coords = coords.len();

    if num_coords == 1 {
        return 1;
    }

    let mut coords_as_isize = coords
        .iter()
        .map(|c| match c {
            Some(value) => *value as isize,
            None => -1,
        })
        .collect_vec();

    coords_as_isize.sort();
    coords_as_isize.windows(2).fold(1, |sum, window| {
        if window[1] - window[0] != 1 {
            sum + 1
        } else {
            sum
        }
    })
}

fn vertical_sides(rows: &HashMap<usize, Vec<&&Square>>) -> usize {
    let existing = rows
        .values()
        .flat_map(|c| c.iter().map(|s| (s.x, s.y)))
        .collect_vec();

    let mut neighbors_by_row = vec![];
    rows.values().for_each(|row| {
        let mut above = vec![];
        let mut below = vec![];
        row.iter().for_each(|s| {
            let v_n = vertical_neighbors_with_ob((s.x, s.y));
            above.push(v_n[0]);
            below.push(v_n[1]);
        });

        neighbors_by_row.push(
            above
                .iter()
                .filter(|n| match (n.0, n.1) {
                    (Some(x), Some(y)) => !existing.contains(&(x, y)),
                    _ => true,
                })
                .map(|v| v.clone())
                .collect_vec(),
        );

        neighbors_by_row.push(
            below
                .iter()
                .filter(|n| match (n.0, n.1) {
                    (Some(x), Some(y)) => !existing.contains(&(x, y)),
                    _ => true,
                })
                .map(|v| v.clone())
                .collect_vec(),
        );
    });

    neighbors_by_row = neighbors_by_row
        .iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.clone())
        .collect_vec();

    neighbors_by_row
        .iter()
        .map(|coords| count_continuous_streaks(coords.iter().map(|c| c.0).collect()))
        .sum()
}

fn horizontal_sides(cols: &HashMap<usize, Vec<&&Square>>) -> usize {
    let existing = cols
        .values()
        .flat_map(|c| c.iter().map(|s| (s.x, s.y)))
        .collect_vec();

    let mut neighbors_by_col = vec![];
    cols.values().for_each(|col| {
        let mut left = vec![];
        let mut right = vec![];
        col.iter().for_each(|s| {
            let h_n = horizontal_neighbors_with_ob((s.x, s.y));
            left.push(h_n[0]);
            right.push(h_n[1]);
        });

        neighbors_by_col.push(
            left.iter()
                .filter(|n| match (n.0, n.1) {
                    (Some(x), Some(y)) => !existing.contains(&(x, y)),
                    _ => true,
                })
                .map(|v| v.clone())
                .collect_vec(),
        );

        neighbors_by_col.push(
            right
                .iter()
                .filter(|n| match (n.0, n.1) {
                    (Some(x), Some(y)) => !existing.contains(&(x, y)),
                    _ => true,
                })
                .map(|v| v.clone())
                .collect_vec(),
        );
    });

    neighbors_by_col = neighbors_by_col
        .iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.clone())
        .collect_vec();

    neighbors_by_col
        .iter()
        .map(|coords| count_continuous_streaks(coords.iter().map(|c| c.1).collect()))
        .sum()
}

fn sides(map: &Map) -> HashMap<usize, usize> {
    let regions = map.regions.values().into_group_map_by(|s| s.region);

    regions
        .iter()
        .map(|(id, squares)| {
            let rows = squares.iter().into_group_map_by(|s| s.y);
            let cols = squares.iter().into_group_map_by(|s| s.x);
            (*id, horizontal_sides(&cols) + vertical_sides(&rows))
        })
        .collect()
}

fn cost(map: &Map) -> usize {
    let ids = map.regions.values().map(|r| r.region).unique();
    let areas_by_id = areas(map);
    let perimeters_by_id = perimeters(map);

    ids.map(|id| areas_by_id.get(&id).unwrap_or(&0) * perimeters_by_id.get(&id).unwrap_or(&0))
        .sum()
}

fn bulk_cost(map: &Map) -> usize {
    let ids = map.regions.values().map(|r| r.region).unique();
    let areas_by_id = areas(map);
    let sides_by_id = sides(map);

    ids.map(|id| areas_by_id.get(&id).unwrap_or(&0) * sides_by_id.get(&id).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let map = parse_input(include_str!("../input-test.txt"));
        assert!(cost(&map) == 1930);
    }

    #[test]
    fn example_2() {
        let map = parse_input(include_str!("../input-test.txt"));
        assert!(bulk_cost(&map) == 1206);
    }

    #[test]
    fn example_big_e() {
        let map = parse_input(include_str!("../input-test-2.txt"));
        assert!(bulk_cost(&map) == 236);
    }
}
