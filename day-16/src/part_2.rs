use std::collections::HashSet;

use crate::{Map, Point, calculate_score, get_neighbors, part_1::a_star_with_turn_cost};

pub fn best_seats(map: &mut Map, goal: Point, best_score: usize, best_path: Vec<Point>) -> usize {
    let mut all_paths: Vec<Vec<Point>> = vec![best_path.clone()];
    let mut new_paths = vec![best_path.clone()];

    while !new_paths.is_empty() {
        let tmp = new_paths.clone();
        new_paths.clear();
        for path in tmp {
            for nodes in path.windows(2) {
                let current = nodes[0];
                let next = nodes[1];
                if get_neighbors(current, map).len() < 3 {
                    continue; // straight line
                }

                map[next.1][next.0] = false;
                let res = a_star_with_turn_cost(map, current, goal);
                if res.is_some() {
                    let res = res.unwrap();
                    let split_off_index = path.iter().position(|n| *n == current).unwrap();
                    let combined_path = [path[..split_off_index].to_vec(), res.1].concat();
                    let score = calculate_score(combined_path.clone());
                    if score == best_score && !all_paths.contains(&combined_path) {
                        all_paths.push(combined_path.clone());
                        new_paths.push(combined_path);
                    }
                }
                map[next.1][next.0] = true;
            }
        }
    }

    let all_nodes = all_paths
        .into_iter()
        .filter(|path| calculate_score(path.to_vec()) == best_score)
        .flatten();

    let unique_nodes: HashSet<Point> = all_nodes.collect();
    unique_nodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{a_star_with_turn_cost, parse_input};

    #[test]
    fn example_2() {
        let (mut map, start, goal) = parse_input(include_str!("../input-test.txt"));
        let (score, path) = a_star_with_turn_cost(&map, start, goal).unwrap();
        assert!(best_seats(&mut map, goal, score, path) == 64)
    }

    #[test]
    fn example_2_small() {
        let (mut map, start, goal) = parse_input(include_str!("../input-test-small.txt"));
        let (score, path) = a_star_with_turn_cost(&map, start, goal).unwrap();
        assert!(best_seats(&mut map, goal, score, path) == 45)
    }
}
