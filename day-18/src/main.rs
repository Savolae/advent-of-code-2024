use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Point = (usize, usize);

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|value| value.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn create_grid(input: &Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let size = if cfg!(test) { 7 } else { 71 };
    let bytes = if cfg!(test) { 12 } else { 1024 };
    let mut map = vec![vec![true; size]; size];
    input
        .iter()
        .take(bytes)
        .for_each(|(x, y)| map[*y][*x] = false);
    map
}

fn get_neighbors(point: Point, map: &Vec<Vec<bool>>) -> Vec<Point> {
    let max = map.len();
    [
        (Some(point.0), Some(point.1 + 1)),
        (Some(point.0), point.1.checked_sub(1)),
        (Some(point.0 + 1), Some(point.1)),
        (point.0.checked_sub(1), Some(point.1)),
    ]
    .into_iter()
    .filter(|(x, y)| x.is_some() && y.is_some())
    .map(|(x, y)| (x.unwrap(), y.unwrap()))
    .filter(|(x, y)| *x < max && *y < max)
    .filter(|n| map[n.1][n.0])
    .collect()
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: &Point) -> Vec<Point> {
    let mut c = current;
    let mut total: Vec<(usize, usize)> = vec![current.clone()];
    while came_from.contains_key(&c) {
        c = came_from.get(&c).unwrap();
        total.insert(0, c.clone());
    }
    total
}

pub fn a_star(map: &Vec<Vec<bool>>) -> Option<Vec<Point>> {
    let start: Point = (0, 0);
    let goal: Point = if cfg!(test) { (6, 6) } else { (70, 70) };
    let mut open_set: HashSet<Point> = HashSet::from([start]);
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut g_score: HashMap<Point, usize> = HashMap::from([(start, 0)]);
    let mut f_score: HashMap<Point, usize> = HashMap::from([(start, 0)]);

    let d = || 1;

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by(|node, other| {
                let node_score = f_score.get(node).or(Some(&usize::MAX));
                let other_score: Option<&usize> = f_score.get(other).or(Some(&usize::MAX));
                node_score.cmp(&other_score)
            })
            .unwrap()
            .clone();

        if current == goal {
            return Some(reconstruct_path(&came_from, &current));
        }

        open_set.remove(&current);

        for n in get_neighbors(current, map) {
            let tentative_g_score = g_score.get(&current).or(Some(&usize::MAX)).unwrap() + d();

            if tentative_g_score <= *g_score.get(&n).or(Some(&usize::MAX)).unwrap() {
                came_from.insert(n, current);
                g_score.insert(n, tentative_g_score);
                f_score.insert(n, tentative_g_score);
                open_set.insert(n);
            }
        }
    }

    None
}

fn find_first_blocking(input: &Vec<(usize, usize)>) -> (usize, usize) {
    let size = if cfg!(test) { 7 } else { 71 };
    let safe_bytes = if cfg!(test) { 12 } else { 1024 };
    let mut map = vec![vec![true; size]; size];
    let input_iter = input.iter();

    input_iter.clone().take(safe_bytes).for_each(|(x, y)| {
        map[*y][*x] = false;
    });

    let mut safe_route = a_star(&map).unwrap();

    input_iter
        .skip(safe_bytes)
        .find(|(x, y)| {
            map[*y][*x] = false;
            if safe_route.contains(&(*x, *y)) {
                let route = a_star(&map);
                if route.is_some() {
                    safe_route = route.unwrap();
                    false
                } else {
                    true
                }
            } else {
                false
            }
        })
        .unwrap()
        .clone()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let map = create_grid(&input);
    let route = a_star(&map).unwrap();
    println!("Part 1: {}", route.len() - 1);
    let blocking = find_first_blocking(&input);
    println!("Part 2: {},{}", blocking.0, blocking.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = parse_input(include_str!("../input-test.txt"));
        let map = create_grid(&input);
        let route = a_star(&map).unwrap();
        assert!(route.len() - 1 == 22)
    }

    #[test]
    fn example_2() {
        let input = parse_input(include_str!("../input-test.txt"));
        let blocking = find_first_blocking(&input);
        assert!(blocking == (6, 1))
    }
}
