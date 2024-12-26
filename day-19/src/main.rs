use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let mut towels: Vec<_> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect();
    towels.sort_by(|a, b| b.len().cmp(&a.len()));
    let designs = lines.map(String::from).collect();
    (towels, designs)
}

fn match_recursive(
    design: String,
    towels: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if cache.contains_key(&design) {
        return *cache.get(&design).unwrap();
    }

    let possible: Vec<String> = towels
        .iter()
        .filter(|t| design.contains(*t))
        .map(|t| t.to_string())
        .collect();

    let res = possible
        .iter()
        .filter(|towel| design.starts_with(*towel))
        .map(|towel| {
            match_recursive(
                design.strip_prefix(towel).unwrap().to_string(),
                &possible,
                cache,
            )
        })
        .sum();

    cache.insert(design, res);
    res
}

fn count_possible(towels: &Vec<String>, designs: &Vec<String>) -> (usize, usize) {
    let mut cache: HashMap<String, usize> = HashMap::new();
    let possible: Vec<_> = designs
        .iter()
        .map(|d| {
            let possible: Vec<String> = towels
                .iter()
                .filter(|t| d.contains(*t))
                .map(|t| t.to_string())
                .collect();

            match_recursive(d.to_string(), &possible, &mut cache)
        })
        .collect();

    (
        possible.iter().filter(|p| **p > 0).count(),
        possible.iter().sum(),
    )
}

fn main() {
    let (towels, designs) = parse_input(include_str!("../input.txt"));
    let (possible, total) = count_possible(&towels, &designs);
    println!("Part 1: {possible}");
    println!("Part 2: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (towels, designs) = parse_input(include_str!("../input-test.txt"));
        assert!(count_possible(&towels, &designs).0 == 6);
    }

    #[test]
    fn example_2() {
        let (towels, designs) = parse_input(include_str!("../input-test.txt"));
        assert!(count_possible(&towels, &designs).1 == 16);
    }
}
