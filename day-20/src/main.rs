use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use itertools::{Itertools, min};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Map = Vec<Vec<bool>>;
type Point = (usize, usize);
type ShortcutMap = HashMap<((usize, usize), (usize, usize)), usize>;

fn get_neighbors(point: Point, map: &Map) -> Vec<Point> {
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

fn manhattan_distance(first: &Point, second: &Point) -> usize {
    first.0.abs_diff(second.0) + first.1.abs_diff(second.1)
}

fn tiles_within_distance(current: &Point, range: usize, max_x: usize, max_y: usize) -> Vec<Point> {
    let top_triangle = (0..=range)
        .flat_map(|y| {
            (current.0.checked_sub(y).unwrap_or_default()..=min([max_x, current.0 + y]).unwrap())
                .map(move |x| (x, min([max_y, current.1 + (range - y)]).unwrap()))
        })
        .collect_vec();

    let bottom_triangle = (0..range)
        .flat_map(|y| {
            (current.0.checked_sub(y).unwrap_or_default()..=min([max_x, current.0 + y]).unwrap())
                .map(move |x| (x, (current.1.checked_sub(range - y).unwrap_or_default())))
        })
        .collect_vec();

    [top_triangle, bottom_triangle].concat().to_vec()
}

fn get_possible_shortcuts(
    map: &Map,
    current: &Point,
    route: &Vec<Point>,
    max_length: usize,
) -> Vec<((usize, usize), (usize, usize))> {
    let current_index = route.iter().position(|p| p == current).unwrap();
    let index_on_path = |point: &Point| route.iter().position(|p| p == point);

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    tiles_within_distance(current, max_length, max_x, max_y)
        .iter()
        .filter(|(x, y)| {
            let index = index_on_path(&(*x, *y));
            map[*y][*x]
                && index.is_some()
                && manhattan_distance(current, &(*x, *y)) <= max_length
                && index.unwrap() > current_index
        })
        .map(|p| (*current, *p))
        .collect_vec()
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

pub fn a_star(map: &Map, start: &Point, goal: &Point) -> Option<Vec<Point>> {
    let mut open_set: HashSet<Point> = HashSet::from([*start]);
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let h = |point: Point| goal.0.abs_diff(point.0) + goal.1.abs_diff(point.1);

    let mut g_score: HashMap<Point, usize> = HashMap::from([(*start, 0)]);
    let mut f_score: HashMap<Point, usize> = HashMap::from([(*start, h(*start))]);

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

        if current == *goal {
            return Some(reconstruct_path(&came_from, &current));
        }

        open_set.remove(&current);

        for n in get_neighbors(current, map) {
            let tentative_g_score = g_score.get(&current).or(Some(&usize::MAX)).unwrap() + 1;
            if tentative_g_score <= *g_score.get(&n).or(Some(&usize::MAX)).unwrap() {
                came_from.insert(n, current);
                g_score.insert(n, tentative_g_score);
                f_score.insert(n, tentative_g_score + h(n));
                open_set.insert(n);
            }
        }
    }

    None
}

fn parse_input(input: &str) -> (Map, Point, Point) {
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let map = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => false,
                    '.' => true,
                    'S' => {
                        start = (x, y);
                        true
                    }
                    'E' => {
                        goal = (x, y);
                        true
                    }
                    _ => panic!("Unknown map symbol"),
                })
                .collect()
        })
        .collect();

    (map, start, goal)
}

fn check_shortcuts(
    map: &Map,
    route: &Vec<Point>,
    max_shortcut_length: usize,
    min_saved_time: usize,
) -> ShortcutMap {
    let shortcuts = Arc::new(Mutex::new(HashMap::new()));

    route.par_iter().for_each(|point| {
        for new_shortcut in get_possible_shortcuts(map, &point, route, max_shortcut_length) {
            let route_around = a_star(map, point, &new_shortcut.1).unwrap_or_default();
            let saved_time = route_around.len() - manhattan_distance(&point, &new_shortcut.1);

            if saved_time < min_saved_time {
                continue;
            }

            {
                let mut locked_shortcuts = shortcuts.lock().unwrap();
                locked_shortcuts.insert(new_shortcut, saved_time);
            }
        }
    });

    shortcuts.lock().unwrap().clone()
}

fn main() {
    let (map, start, end) = parse_input(include_str!("../input.txt"));
    let route = a_star(&map, &start, &end).unwrap();
    let shortcuts = check_shortcuts(&map, &route, 2, 100);
    println!("Part 1: {}", shortcuts.len());
    let shortcuts = check_shortcuts(&map, &route, 20, 100);
    println!("Part 2: {}", shortcuts.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (map, start, end) = parse_input(include_str!("../input-test.txt"));
        let route = a_star(&map, &start, &end).unwrap();
        let shortcuts = check_shortcuts(&map, &route, 2, 2);

        assert!(shortcuts.len() == 44)
    }

    #[test]
    fn example_2() {
        let (map, start, end) = parse_input(include_str!("../input-test.txt"));
        let route = a_star(&map, &start, &end).unwrap();
        let shortcuts = check_shortcuts(&map, &route, 20, 50);

        assert!(dbg!(shortcuts.len()) == 285)
    }
}
