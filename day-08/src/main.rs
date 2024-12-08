use itertools::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Antenna {
    x: usize,
    y: usize,
    symbol: char,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    width: usize,
    height: usize,
    antennas: Vec<Antenna>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let lines = input.lines().filter(|line| line.len() > 0).collect_vec();
        let antennas = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, symbol)| match symbol != '.' {
                        true => Some(Antenna { x, y, symbol }),
                        false => None,
                    })
            })
            .collect();
        let height = lines.len();
        let width = lines[0].len();

        Map {
            width,
            height,
            antennas,
        }
    }

    pub fn frequencies(&self) -> Vec<char> {
        self.antennas.iter().map(|a| a.symbol).unique().collect()
    }

    pub fn antennas_with_frequency(&self, frequency: &char) -> Vec<&Antenna> {
        self.antennas
            .iter()
            .filter(|a| a.symbol == *frequency)
            .collect()
    }

    pub fn in_bounds(&self, coord: &(usize, usize)) -> bool {
        coord.0 < self.width && coord.1 < self.height
    }
}

fn main() {
    let map = Map::new(include_str!("../input.txt"));
    println!("Part 1: {}", count_antinodes(&map));
    println!("Part 2: {}", count_antinodes_with_harmonics(&map));
}

fn calculate_antinode(
    antenna: &Antenna,
    other: &Antenna,
    x_dist: usize,
    y_dist: usize,
) -> Option<(usize, usize)> {
    let antinode_x = if antenna.x < other.x {
        antenna.x.checked_sub(x_dist)
    } else {
        Some(antenna.x + x_dist)
    };

    let antinode_y = if antenna.y < other.y {
        antenna.y.checked_sub(y_dist)
    } else {
        Some(antenna.y + y_dist)
    };

    match (antinode_x, antinode_y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

fn antinode(antenna: &Antenna, other: &Antenna) -> Option<(usize, usize)> {
    if antenna == other {
        return None;
    }

    let x_dist = antenna.x.abs_diff(other.x);
    let y_dist = antenna.y.abs_diff(other.y);

    calculate_antinode(antenna, other, x_dist, y_dist)
}

fn antinodes_with_harmonics(antenna: &Antenna, other: &Antenna, map: &Map) -> Vec<(usize, usize)> {
    let harmonics_generator = (0..).map(|multiplier| {
        if antenna == other {
            return None;
        }

        let x_dist = antenna.x.abs_diff(other.x) * multiplier;
        let y_dist = antenna.y.abs_diff(other.y) * multiplier;

        calculate_antinode(antenna, other, x_dist, y_dist)
    });

    harmonics_generator
        .take_while(|node| node.is_some_and(|node| map.in_bounds(&node)))
        .filter_map(|node| node)
        .collect_vec()
}

fn antinodes_for_antenna(antenna: &Antenna, others: &Vec<&Antenna>) -> Vec<(usize, usize)> {
    others
        .iter()
        .filter_map(|other| antinode(antenna, other))
        .collect()
}

fn antinodes_for_antenna_with_harmonics(
    antenna: &Antenna,
    others: &Vec<&Antenna>,
    map: &Map,
) -> Vec<(usize, usize)> {
    others
        .iter()
        .flat_map(|other| antinodes_with_harmonics(antenna, other, map))
        .collect()
}

fn count_antinodes(map: &Map) -> usize {
    map.frequencies()
        .iter()
        .flat_map(|f| {
            let antennas = map.antennas_with_frequency(f);
            antennas
                .iter()
                .flat_map(|antenna| antinodes_for_antenna(antenna, &antennas))
                .filter(|node| map.in_bounds(node))
                .collect_vec()
        })
        .unique()
        .count()
}

fn count_antinodes_with_harmonics(map: &Map) -> usize {
    map.frequencies()
        .iter()
        .flat_map(|f| {
            let antennas = map.antennas_with_frequency(f);
            antennas
                .iter()
                .flat_map(|antenna| antinodes_for_antenna_with_harmonics(antenna, &antennas, map))
                .collect_vec()
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(count_antinodes(&Map::new(include_str!("../input-test.txt"))) == 14);
    }

    #[test]
    fn example_2() {
        assert!(count_antinodes_with_harmonics(&Map::new(include_str!("../input-test.txt"))) == 34);
    }
}
