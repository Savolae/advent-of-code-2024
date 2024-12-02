use std::cmp::Ordering;

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let part_1_result = part_1(&input);
    println!("Part 1: {part_1_result}");
    let part_2_result = part_2(&input);
    println!("Part 2: {part_2_result}");
}

fn parse_input(input: &'static str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|value| value.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn is_safe_line(line: &Vec<usize>) -> bool {
    let direction: Ordering = line[0].cmp(&line[1]);
    if direction == Ordering::Equal {
        return false;
    }

    let check = |first: usize, second: usize| {
        first.cmp(&second) == direction && first.abs_diff(second) <= 3
    };

    line.windows(2).all(|window| check(window[0], window[1]))
}

fn is_safe_line_with_damping(line: &Vec<usize>) -> bool {
    let remove_index_and_check = |index| {
        let mut new = line.clone();
        new.remove(index);
        is_safe_line(&new)
    };

    (0..line.len()).any(remove_index_and_check)
}

fn part_1(input: &Vec<Vec<usize>>) -> usize {
    input.iter().filter(|line| is_safe_line(line)).count()
}

fn part_2(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .filter(|line| is_safe_line_with_damping(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = parse_input(include_str!("../input-test-1.txt"));
        assert!(part_1(&input) == 2);
    }

    #[test]
    fn part_2_example() {
        let input = parse_input(include_str!("../input-test-1.txt"));
        assert!(part_2(&input) == 4);
    }
}
