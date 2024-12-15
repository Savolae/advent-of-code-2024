use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Free,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

type Map = Vec<Vec<State>>;

pub fn run() {
    let (initial_position, mut map, moves) = parse_input(include_str!("../input.txt"));
    println!("Part 2: {}", part_2(&mut map, initial_position, moves));
}

fn parse_input(input: &str) -> ((usize, usize), Map, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let map_part = parts.next().unwrap();
    let moves_part = parts.next().unwrap();
    let mut robot = (0, 0);
    let mut check_map = |x, y, c| match c {
        '#' => [State::Wall, State::Wall],
        'O' => [State::BoxLeft, State::BoxRight],
        '.' => [State::Free, State::Free],
        '@' => {
            robot = (x * 2, y);
            [State::Free, State::Free]
        }
        _ => panic!("Unexpected map symbol at ({x}, {y}): {c}"),
    };

    let map = map_part
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, col)| {
            col.chars()
                .enumerate()
                .flat_map(|(x, c)| check_map(x, y, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let moves = moves_part
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => Move::Left,
                '>' => Move::Right,
                '^' => Move::Up,
                'v' => Move::Down,
                _ => panic!("Unexpected move symbol: {c}"),
            })
        })
        .collect::<Vec<_>>();

    (robot, map, moves)
}

fn get_target(position: (usize, usize), direction: Move) -> (usize, usize) {
    match direction {
        Move::Up => (position.0, position.1 - 1),
        Move::Down => (position.0, position.1 + 1),
        Move::Left => (position.0 - 1, position.1),
        Move::Right => (position.0 + 1, position.1),
    }
}

fn get_boxes_in_front(
    map: &mut Map,
    position: (usize, usize),
    direction: Move,
) -> Option<Vec<(usize, usize)>> {
    let target = get_target(position, direction);
    let target_state = map[target.1][target.0];
    let other_target = match target_state {
        State::Free => None,
        State::BoxLeft => Some((target.0 + 1, target.1)),
        State::BoxRight => Some((target.0 - 1, target.1)),
        State::Wall => None,
    };

    if target_state == State::Wall {
        return None;
    }

    if other_target.is_some_and(|other| map[other.1][other.0] == State::Wall) {
        return None;
    }

    if target_state == State::Free {
        if other_target.is_none()
            || other_target.is_some_and(|other| map[other.1][other.0] == State::Free)
        {
            return Some(vec![]);
        }
    }

    let Some(box_in_front) = get_boxes_in_front(map, target, direction) else {
        return None;
    };

    if other_target.is_none() {
        return Some(
            vec![vec![target], box_in_front]
                .iter()
                .flatten()
                .unique()
                .map(|b| b.clone())
                .collect(),
        );
    }

    let other_target = other_target.unwrap();

    let Some(other_half) = (if other_target != position {
        get_boxes_in_front(map, other_target, direction)
    } else {
        Some(vec![])
    }) else {
        return None;
    };

    Some(
        vec![vec![target, other_target], box_in_front, other_half]
            .iter()
            .flatten()
            .unique()
            .map(|b| b.clone())
            .collect(),
    )
}

fn try_to_move(map: &mut Map, position: (usize, usize), direction: Move) -> (usize, usize) {
    let target = get_target(position, direction);
    if map[target.1][target.0] == State::Free {
        return target;
    }

    if map[target.1][target.0] == State::Wall {
        return position;
    }

    let other_half = if map[target.1][target.0] == State::BoxLeft {
        (target.0 + 1, target.1)
    } else {
        (target.0 - 1, target.1)
    };

    let Some(clump_of_boxes) = get_boxes_in_front(map, target, direction) else {
        return position;
    };

    let Some(other_clump_of_boxes) = get_boxes_in_front(map, other_half, direction) else {
        return position;
    };

    let all_boxes = vec![
        vec![target, other_half],
        clump_of_boxes,
        other_clump_of_boxes,
    ]
    .concat();
    let all_boxes = all_boxes.iter().unique().collect_vec();

    let new_positions = all_boxes
        .iter()
        .map(|b| (get_target(**b, direction), map[b.1][b.0]))
        .collect_vec();
    let new_free_positions = all_boxes
        .iter()
        .filter(|b| !new_positions.iter().map(|(pos, _)| pos).contains(*b))
        .collect_vec();

    for (pos, state) in new_positions {
        map[pos.1][pos.0] = state;
    }

    for pos in new_free_positions {
        map[pos.1][pos.0] = State::Free;
    }

    target
}

fn run_moves(map: &mut Map, initial_position: (usize, usize), moves: Vec<Move>) {
    let mut position = initial_position;
    for direction in moves {
        position = try_to_move(map, position, direction);
    }
}

fn _print_map(map: &Map, robot_pos: (usize, usize)) {
    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, s)| {
            if (x, y) == robot_pos {
                print!("@")
            } else {
                match s {
                    State::Free => print!("."),
                    State::BoxLeft => print!("["),
                    State::BoxRight => print!("]"),
                    State::Wall => print!("#"),
                }
            }
        });
        println!();
    });
}

fn part_2(map: &mut Map, initial_position: (usize, usize), moves: Vec<Move>) -> usize {
    run_moves(map, initial_position, moves);
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, s)| match s {
                State::BoxLeft => 100 * y + x,
                State::BoxRight | State::Free | State::Wall => 0,
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_large() {
        let (initial_position, mut map, moves) =
            parse_input(include_str!("../input-test-large.txt"));
        assert!(part_2(&mut map, initial_position, moves) == 9021);
    }
}
