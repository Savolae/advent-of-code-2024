#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Free,
    Box,
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
    println!("Part 1: {}", part_1(&mut map, initial_position, moves));
}

fn parse_input(input: &str) -> ((usize, usize), Map, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let map_part = parts.next().unwrap();
    let moves_part = parts.next().unwrap();
    let mut robot = (0, 0);
    let mut check_map = |x, y, c| match c {
        '#' => State::Wall,
        'O' => State::Box,
        '.' => State::Free,
        '@' => {
            robot = (x, y);
            State::Free
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
                .map(|(x, c)| check_map(x, y, c))
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

fn try_push(map: &mut Map, position: (usize, usize), direction: Move) -> bool {
    let target = get_target(position, direction);
    match map[target.1][target.0] {
        State::Free => {
            map[target.1][target.0] = State::Box;
            map[position.1][position.0] = State::Free;
            true
        }
        State::Wall => false,
        State::Box => {
            let can_move = try_push(map, target, direction);
            match can_move {
                true => {
                    map[target.1][target.0] = State::Box;
                    map[position.1][position.0] = State::Free;
                    true
                }
                false => false,
            }
        }
    }
}

fn try_to_move(map: &mut Map, position: (usize, usize), direction: Move) -> (usize, usize) {
    let target = get_target(position, direction);
    match map[target.1][target.0] {
        State::Free => return target,
        State::Box => match try_push(map, target, direction) {
            true => target,
            false => position,
        },
        State::Wall => position,
    }
}

fn run_moves(map: &mut Map, initial_position: (usize, usize), moves: Vec<Move>) {
    let mut position = initial_position;
    for direction in moves {
        position = try_to_move(map, position, direction);
    }
}

fn part_1(map: &mut Map, initial_position: (usize, usize), moves: Vec<Move>) -> usize {
    run_moves(map, initial_position, moves);
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, s)| match s {
                State::Box => 100 * y + x,
                State::Free | State::Wall => 0,
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small() {
        let (initial_position, mut map, moves) =
            parse_input(include_str!("../input-test-small.txt"));
        assert!(part_1(&mut map, initial_position, moves) == 2028);
    }

    #[test]
    fn example_large() {
        let (initial_position, mut map, moves) =
            parse_input(include_str!("../input-test-large.txt"));
        assert!(part_1(&mut map, initial_position, moves) == 10092);
    }
}
