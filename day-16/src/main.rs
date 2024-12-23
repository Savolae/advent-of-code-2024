mod part_1;
mod part_2;

use part_1::a_star_with_turn_cost;
use part_2::best_seats;

type Map = Vec<Vec<bool>>;
type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

fn get_neighbors(point: Point, map: &Map) -> Vec<Point> {
    [
        (point.0, point.1 + 1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1),
        (point.0 - 1, point.1),
    ]
    .into_iter()
    .filter(|n| map[n.1][n.0])
    .collect()
}

fn get_direction(from: Point, to: Point) -> Direction {
    match (
        to.0 as isize - from.0 as isize,
        to.1 as isize - from.1 as isize,
    ) {
        (0, 1) => Direction::Down,
        (0, -1) => Direction::Up,
        (1, 0) => Direction::Right,
        (-1, 0) => Direction::Left,
        e => panic!("Not a neighbor: {e:?} from {from:?} to {to:?}"),
    }
}

fn calculate_score(path: Vec<Point>) -> usize {
    let mut score = 0;
    let mut previus: Option<Point> = None;

    path.windows(2).for_each(|window| {
        let current = window[0];
        let next = window[1];

        let current_direction = if previus.is_none() {
            Direction::Right
        } else {
            get_direction(previus.unwrap(), current)
        };

        let next_direction = get_direction(current, next);

        score += if current_direction == next_direction {
            1
        } else {
            1001
        };

        previus = Some(current);
    });

    score
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

fn main() {
    let (mut map, start, goal) = parse_input(include_str!("../input.txt"));
    let (score, path) = a_star_with_turn_cost(&map, start, goal).unwrap();
    println!("Part 1: {}", score);
    println!("Part 2: {}", best_seats(&mut map, goal, score, path));
}
