use std::{collections::HashMap, iter::successors};

fn main() {
    let stones = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", blink_times(25, &stones));
    println!("Part 2: {}", blink_times(75, &stones));
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .map(|v| (v, 1))
        .collect()
}

fn split(n: usize, length: usize) -> [usize; 2] {
    let denominator = 10_usize.pow((length / 2) as u32);
    [n / denominator, n % denominator]
}

fn change_stone(stone: (usize, usize)) -> Vec<(usize, usize)> {
    if stone.0 == 0 {
        return vec![(1, stone.1)];
    }

    let digits = count_digits(stone.0);
    if digits % 2 == 0 {
        let [first, second] = split(stone.0, digits);
        vec![(first, stone.1), (second, stone.1)]
    } else {
        vec![(stone.0 * 2024, stone.1)]
    }
}

fn count_digits(n: usize) -> usize {
    successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count()
}

fn update_multipliers(stones: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        *map.entry(stone.0).or_default() += stone.1;
    }
    map.iter()
        .map(|(stone, amount)| (*stone, *amount))
        .collect()
}

fn blink_times(n: usize, stones: &Vec<(usize, usize)>) -> usize {
    let mut res = stones.clone();
    for _ in 0..n {
        let new_stones = res.iter().flat_map(|s| change_stone(*s)).collect();
        res = update_multipliers(new_stones);
    }

    res.iter().map(|(_, amount)| amount).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(blink_times(25, &parse_input(include_str!("../input-test.txt"))) == 55312)
    }
}
