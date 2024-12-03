use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
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

fn part_2(input: &str) -> u64 {
    let Ok(remover) = Regex::new(r"don\'t\(\)[\S\s]*?do\(\)") else {
        return 0;
    };

    let Ok(ending_remover) = Regex::new(r"don\'t\(\)[\S\s]*$") else {
        return 0;
    };

    let new_input = remover.replace_all(input, "");
    let new_input = ending_remover.replace_all(&new_input, "");
    part_1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(include_str!("../input-test.txt")), 161);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(include_str!("../input-test-2.txt")), 48);
    }
}
