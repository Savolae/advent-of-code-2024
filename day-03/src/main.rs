use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let part_1_result = part_1(input);
    println!("Part 1: {}", part_1_result);
}

fn part_1(input: &str) -> u64 {
    let Ok(re) = Regex::new(r"mul\((\d+),(\d+)\)") else {
        return 0;
    };

    re.captures_iter(input)
        .map(|cap| {
            cap[1].parse::<u64>().unwrap_or_default() * cap[2].parse::<u64>().unwrap_or_default()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(include_str!("../input-test.txt")), 161);
    }
}
