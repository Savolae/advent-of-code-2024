use itertools::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Equation {
    total: u64,
    values: Vec<u64>,
}

impl Equation {
    pub fn new(input: &str) -> Self {
        let (total, values) = input.split(": ").collect_tuple().unwrap();
        let total = total.parse().unwrap();
        let values = values
            .split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect_vec();
        Equation { total, values }
    }
}

fn concatenate(first: u64, second: u64) -> u64 {
    format!("{first}{second}").parse().unwrap()
}

fn get_all_combinations(values: &Vec<u64>) -> Vec<u64> {
    let Some((last, elements)) = values.split_last() else {
        return vec![];
    };

    if elements.is_empty() {
        return vec![*last];
    }

    get_all_combinations(&elements.to_vec())
        .iter()
        .flat_map(|v| vec![v + last, v * last, concatenate(*v, *last)])
        .collect()
}

fn is_possible(remaining: u64, values: &Vec<u64>) -> bool {
    let Some((last, elements)) = values.split_last() else {
        return false;
    };

    if elements.is_empty() {
        return remaining == *last;
    }

    let elements = elements.to_vec();

    let can_be_divided = remaining % last == 0 && is_possible(remaining / last, &elements);

    can_be_divided || remaining > *last && is_possible(remaining - last, &elements)
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(Equation::new)
        .collect()
}

fn part_1(equations: &Vec<Equation>) -> u64 {
    equations
        .iter()
        .filter(|eq| is_possible(eq.total, &eq.values))
        .map(|eq| eq.total)
        .sum()
}

fn part_2(equations: &Vec<Equation>) -> u64 {
    let (part_1_possible, rest): (Vec<_>, Vec<_>) = equations
        .iter()
        .partition(|eq| is_possible(eq.total, &eq.values));

    rest.iter()
        .filter(|eq| get_all_combinations(&eq.values).contains(&eq.total))
        .chain(part_1_possible.iter())
        .map(|eq| eq.total)
        .sum()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = parse_input(include_str!("../input-test.txt"));
        assert!(part_1(&input) == 3749);
    }

    #[test]
    fn example_2() {
        let input = parse_input(include_str!("../input-test.txt"));
        assert!(part_2(&input) == 11387);
    }
}
