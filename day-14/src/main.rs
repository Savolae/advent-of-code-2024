use std::cmp;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn step(self, max_x: isize, max_y: isize) -> Robot {
        let mut new_x = self.position.0;
        let neg_x = self.velocity.0 < 0;
        for _ in 0..self.velocity.0.abs() {
            new_x += if neg_x { -1 } else { 1 };
            if new_x < 0 {
                new_x = max_x - 1;
            } else if new_x == max_x {
                new_x = 0;
            }
        }

        let mut new_y = self.position.1;
        let neg_y = self.velocity.1 < 0;
        for _ in 0..self.velocity.1.abs() {
            new_y += if neg_y { -1 } else { 1 };
            if new_y < 0 {
                new_y = max_y - 1;
            } else if new_y == max_y {
                new_y = 0;
            }
        }
        Robot {
            position: (new_x, new_y),
            velocity: self.velocity,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    height: isize,
    width: isize,
    robots: Vec<Robot>,
}

impl Map {
    fn step(&mut self) {
        self.robots = self
            .robots
            .iter()
            .map(|robot| robot.step(self.width, self.height))
            .collect();
    }

    fn safety_factor(&self) -> usize {
        let top_left = self
            .robots
            .iter()
            .filter(|r| r.position.0 < self.width / 2 && r.position.1 < self.height / 2)
            .count();

        let bottom_left = self
            .robots
            .iter()
            .filter(|r| r.position.0 < self.width / 2 && r.position.1 > self.height / 2)
            .count();

        let top_right = self
            .robots
            .iter()
            .filter(|r| r.position.0 > self.width / 2 && r.position.1 < self.height / 2)
            .count();

        let bottom_right = self
            .robots
            .iter()
            .filter(|r| r.position.0 > self.width / 2 && r.position.1 > self.height / 2)
            .count();

        top_left * bottom_left * top_right * bottom_right
    }

    fn print(&self, step: usize) -> bool {
        let rows = self.robots.iter().into_group_map_by(|r| r.position.1);
        let rows_with_lines = rows
            .values()
            .map(|row| {
                row.iter()
                    .map(|r| r.position.0)
                    .unique()
                    .sorted()
                    .collect_vec()
            })
            .filter(|row| {
                let mut line_len = 0;
                let mut best_line_len = 0;
                row.windows(2).for_each(|window| {
                    if window[1] - window[0] == 1 {
                        line_len += 1;
                    } else {
                        best_line_len = cmp::max(best_line_len, line_len);
                        line_len = 0
                    }
                });
                best_line_len = cmp::max(best_line_len, line_len);
                best_line_len > 10
            })
            .count()
            > 0;

        if !rows_with_lines {
            return false;
        };

        println!("Step: {step}");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.robots.iter().filter(|r| r.position == (x, y)).count() > 0 {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!()
        }
        true
    }
}

fn main() {
    let mut map = parse_input(include_str!("../input.txt"), false);
    println!("Part 1: {}", simulate_steps(100, &mut map, false));
    let mut map = parse_input(include_str!("../input.txt"), false);
    println!("Part 2:");
    simulate_steps(10000, &mut map, true);
}

fn parse_input(input: &str, test: bool) -> Map {
    let robots: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut position = parts
                .next()
                .unwrap()
                .strip_prefix("p=")
                .unwrap()
                .split(",")
                .map(|p| p.parse::<isize>().unwrap());
            let mut velocity = parts
                .next()
                .unwrap()
                .strip_prefix("v=")
                .unwrap()
                .split(",")
                .map(|p| p.parse::<isize>().unwrap());
            let position = (position.next().unwrap(), position.next().unwrap());
            let velocity = (velocity.next().unwrap(), velocity.next().unwrap());
            Robot { position, velocity }
        })
        .collect();

    Map {
        width: if test { 11 } else { 101 },
        height: if test { 7 } else { 103 },
        robots,
    }
}

fn simulate_steps(steps: usize, map: &mut Map, print: bool) -> usize {
    for i in 0..steps {
        if print {
            map.print(i);
        }
        map.step();
    }

    map.safety_factor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(
            simulate_steps(
                100,
                &mut parse_input(include_str!("../input-test.txt"), true),
                false
            ) == 12
        );
    }
}
