use itertools::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rule {
    After,
    Before,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OrderRule(usize, usize);

impl OrderRule {
    pub fn new(rule: &str) -> Self {
        let (first, second) = rule
            .split("|")
            .map(|value| value.parse().unwrap_or_default())
            .collect_tuple()
            .unwrap_or_default();
        OrderRule(first, second)
    }

    pub fn involves(self, value: &usize) -> Option<(Rule, usize)> {
        match (self.0 == *value, self.1 == *value) {
            (true, false) => Some((Rule::Before, self.1)),
            (false, true) => Some((Rule::After, self.0)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RuleSet {
    rules: Vec<OrderRule>,
}

impl RuleSet {
    pub fn new(rules: &str) -> Self {
        RuleSet {
            rules: rules
                .lines()
                .filter(|line| line.len() > 0)
                .map(OrderRule::new)
                .collect(),
        }
    }

    fn get_rules_with_value(&self, value: &usize) -> Vec<(Rule, usize)> {
        self.rules
            .iter()
            .filter_map(|rule| rule.involves(value))
            .collect()
    }

    fn fulfills_rule(&self, update: &Vec<usize>, index: usize, rule: &(Rule, usize)) -> bool {
        let (rule, other) = rule;
        let comparison = match rule {
            Rule::After => |index, pos| pos < index,
            Rule::Before => |index, pos| pos > index,
        };

        update
            .iter()
            .position(|v| v == other)
            .is_none_or(|pos| comparison(index, pos))
    }

    pub fn is_valid_update(&self, update: &Vec<usize>) -> bool {
        update.iter().enumerate().all(|(i, value)| {
            self.get_rules_with_value(value)
                .iter()
                .all(|rule| self.fulfills_rule(update, i, rule))
        })
    }

    pub fn find_failing_index(&self, update: &Vec<usize>) -> Option<(usize, usize)> {
        for (index, value) in update.iter().enumerate() {
            let rules = self.get_rules_with_value(value);
            for (rule, other) in rules {
                let Some(pos_of_other) = update.iter().position(|v| *v == other) else {
                    continue;
                };

                if !match rule {
                    Rule::After => pos_of_other < index,
                    Rule::Before => pos_of_other > index,
                } {
                    return Some((index, pos_of_other));
                }
            }
        }

        None
    }
}

fn fix_update(update: &Vec<usize>, rules: &RuleSet) -> Vec<usize> {
    let mut tmp = update.clone();
    match rules.find_failing_index(&update) {
        Some((first, second)) => {
            tmp.swap(first, second);
            fix_update(&tmp, rules)
        }
        None => tmp,
    }
}

fn parse_updates(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(",")
                .filter_map(|value| value.parse().ok())
                .collect()
        })
        .collect()
}

fn part_1(rules: &RuleSet, updates: &Vec<Vec<usize>>) -> usize {
    updates
        .iter()
        .filter(|update| rules.is_valid_update(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_2(rules: &RuleSet, updates: &Vec<Vec<usize>>) -> usize {
    updates
        .iter()
        .filter(|update| !rules.is_valid_update(update))
        .map(|update| fix_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let (rule_part, update_part) = input.split("\n\n").take(2).collect_tuple().unwrap();
    let rules = RuleSet::new(rule_part);
    let updates = parse_updates(update_part);
    println!("Part 1: {}", part_1(&rules, &updates));
    println!("Part 2: {}", part_2(&rules, &updates));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = include_str!("../input-test.txt");
        let (rule_part, update_part) = input
            .split(&format!("\n\n"))
            .take(2)
            .collect_tuple()
            .unwrap();
        let rules = RuleSet::new(rule_part);
        let updates = parse_updates(update_part);
        assert!(part_1(&rules, &updates) == 143);
    }

    #[test]
    fn example_2() {
        let input = include_str!("../input-test.txt");
        let (rule_part, update_part) = input
            .split(&format!("\n\n"))
            .take(2)
            .collect_tuple()
            .unwrap();
        let rules = RuleSet::new(rule_part);
        let updates = parse_updates(update_part);
        assert!(dbg!(part_2(&rules, &updates)) == 123);
    }
}
