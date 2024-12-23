use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::{Direction, Map, Point, get_direction, get_neighbors};

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: &Point) -> Vec<Point> {
    let mut c = current;
    let mut total: Vec<(usize, usize)> = vec![current.clone()];
    while came_from.contains_key(&c) {
        c = came_from.get(&c).unwrap();
        total.insert(0, c.clone());
    }
    total
}

pub fn a_star_with_turn_cost(map: &Map, start: Point, goal: Point) -> Option<(usize, Vec<Point>)> {
    let mut open_set: HashSet<Point> = HashSet::from([start]);
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let h = |point: Point| goal.0.abs_diff(point.0) + goal.1.abs_diff(point.1);

    let mut g_score: HashMap<Point, usize> = HashMap::from([(start, 0)]);
    let mut f_score: HashMap<Point, usize> = HashMap::from([(start, h(start))]);

    let d = |c: Point, n: Point, p: Point| {
        if c == start {
            return if get_direction(c, n) != Direction::Right {
                1001
            } else {
                1
            };
        }
        let turn = get_direction(p, c) != get_direction(c, n);
        if turn { 1001 } else { 1 }
    };

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
            return Some((
                *f_score.get(&current).unwrap(),
                reconstruct_path(&came_from, &current),
            ));
        }

        open_set.remove(&current);

        for n in get_neighbors(current, map) {
            let tentative_g_score = g_score.get(&current).or(Some(&usize::MAX)).unwrap()
                + d(current, n, *came_from.get(&current).unwrap_or(&start));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    #[test]
    fn example_1() {
        let (map, start, goal) = parse_input(include_str!("../input-test.txt"));
        assert!(a_star_with_turn_cost(&map, start, goal).unwrap().0 == 11048)
    }
}
