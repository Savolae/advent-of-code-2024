use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Agent {
    position: (usize, usize),
    starting_position: (usize, usize),
    direction: Direction,
    history: HashSet<(usize, usize)>,
    history_with_direction: HashSet<(usize, usize, Direction)>,
}

impl Agent {
    pub fn new(position: (usize, usize)) -> Self {
        Agent {
            position,
            starting_position: position.clone(),
            direction: Direction::Up,
            history: [position].into(),
            history_with_direction: [].into(),
        }
    }

    pub fn position_and_direction(&self) -> (usize, usize, Direction) {
        (self.position.0, self.position.1, self.direction)
    }

    pub fn next_move(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn next_tile(&self) -> Option<(usize, usize)> {
        let movement = self.next_move();
        match (
            self.position.0.checked_add_signed(movement.0),
            self.position.1.checked_add_signed(movement.1),
        ) {
            (Some(y), Some(x)) => Some((y, x)),
            _ => None,
        }
    }

    pub fn move_forward(&mut self) {
        self.position = self.next_tile().unwrap();
        self.history.insert(self.position);
    }

    fn get_turn_direction(&self) -> Direction {
        match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn(&mut self) {
        self.direction = self.get_turn_direction();
        self.history_with_direction
            .insert(self.position_and_direction());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    tiles: Vec<Vec<bool>>,
    agent: Agent,
    height: usize,
    width: usize,
    loops: HashSet<(usize, usize)>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut agent_pos = (0, 0);
        let tiles = input
            .lines()
            .filter(|line| line.len() > 0)
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '^' => {
                            agent_pos = (y, x);
                            true
                        }
                        '#' => false,
                        _ => true,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = tiles.len();
        let width = tiles[0].len();

        Map {
            tiles,
            height,
            width,
            agent: Agent::new(agent_pos),
            loops: [].into(),
        }
    }

    fn will_agent_walk_off(&self) -> bool {
        let movement = self.agent.next_move();
        let up_or_left = match (
            self.agent.position.0.checked_add_signed(movement.0),
            self.agent.position.1.checked_add_signed(movement.1),
        ) {
            (_, None) => true,
            (None, _) => true,
            _ => false,
        };

        let down_or_right = match (
            self.agent.position.0.saturating_add_signed(movement.0) >= self.height,
            self.agent.position.1.saturating_add_signed(movement.1) >= self.width,
        ) {
            (true, _) => true,
            (_, true) => true,
            _ => false,
        };

        up_or_left || down_or_right
    }

    fn is_free(&self, tile: (usize, usize)) -> bool {
        self.tiles[tile.0][tile.1]
    }

    fn add_obstacle_and_check_loop(&mut self) {
        let Some(obstacle_location) = self.agent.next_tile() else {
            return;
        };

        match (
            obstacle_location.0 >= self.height,
            obstacle_location.1 >= self.width,
        ) {
            (false, false) => (),
            _ => return,
        }

        if obstacle_location == self.agent.starting_position {
            return;
        }

        if self.agent.history.contains(&obstacle_location) {
            return;
        }

        let old_agent = self.agent.clone();
        let mut looping = false;

        let old_tile = self.tiles[obstacle_location.0][obstacle_location.1];
        if !old_tile {
            return;
        }

        self.tiles[obstacle_location.0][obstacle_location.1] = false;

        while !self.will_agent_walk_off() {
            match self.is_free(self.agent.next_tile().unwrap()) {
                true => self.agent.move_forward(),
                false => {
                    let pos = self.agent.position;
                    if self.agent.history_with_direction.contains(&(
                        pos.0,
                        pos.1,
                        self.agent.get_turn_direction(),
                    )) {
                        looping = true;
                        break;
                    }
                    self.agent.turn();
                }
            }
        }

        if looping {
            self.loops.insert(obstacle_location);
        }

        self.tiles[obstacle_location.0][obstacle_location.1] = old_tile;
        self.agent = old_agent;
    }

    pub fn simulate(&mut self, add_obstacles: bool) {
        while !self.will_agent_walk_off() {
            match self.is_free(self.agent.next_tile().unwrap()) {
                true => self.agent.move_forward(),
                false => self.agent.turn(),
            }
            if add_obstacles {
                self.add_obstacle_and_check_loop();
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut map = Map::new(input);
    map.simulate(false);
    map.agent.history.len()
}

fn part_2(input: &str) -> usize {
    let mut map = Map::new(input);
    map.simulate(true);
    map.loops.len()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(part_1(include_str!("../input-test.txt")) == 41);
    }

    #[test]
    fn example_2() {
        assert!(part_2(include_str!("../input-test.txt")) == 6);
    }
}
