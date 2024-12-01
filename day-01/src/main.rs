type VecPair = (Vec<u64>, Vec<u64>);

fn main() {
    let input = parse_input(include_str!("../input.txt").to_string());
    let part_1_result = part_1(input.clone());
    println!("Part 1: {part_1_result}");
    let part_2_result = part_2(input);
    println!("Part 2: {part_2_result}");
}

fn parse_input(input: String) -> VecPair {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.split_whitespace())
        .map(|mut values| {
            (
                values.next().unwrap().parse::<u64>().unwrap(),
                values.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn part_1(input: VecPair) -> u64 {
    let (mut left_col, mut right_col) = input;
    left_col.sort();
    right_col.sort();
    left_col
        .into_iter()
        .zip(right_col.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part_2(input: VecPair) -> u64 {
    let (left_col, right_col) = input;
    left_col
        .into_iter()
        .map(|value| value * right_col.iter().filter(|v| **v == value).count() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = parse_input(include_str!("../input-test-1.txt").to_string());
        assert!(part_1(input) == 11)
    }

    #[test]
    fn part_2_example() {
        let input = parse_input(include_str!("../input-test-1.txt").to_string());
        assert!(part_2(input) == 31)
    }
}
