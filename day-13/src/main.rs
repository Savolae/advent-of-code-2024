use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LinearEquation {
    a_mul: isize,
    b_mul: isize,
    total: isize,
}

type System = (LinearEquation, LinearEquation);

fn main() {
    let systems = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", total_cost(&systems, false));
    println!("Part 2: {}", total_cost(&systems, true));
}

fn parse_input(input: &str) -> Vec<System> {
    input
        .split(&format!("\n\n"))
        .map(|chunk| {
            let (a_line, b_line, prize_line) = chunk
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let relevant_part = line.split(": ").skip(1).next().unwrap();
                    let (x, y) = relevant_part
                        .split(", ")
                        .map(|part| {
                            part.split("+")
                                .skip(1)
                                .next()
                                .or(part.split("=").skip(1).next())
                                .unwrap()
                        })
                        .filter_map(|v| v.parse::<isize>().ok())
                        .collect_tuple()
                        .unwrap();

                    (x, y)
                })
                .collect_tuple()
                .unwrap();
            (
                LinearEquation {
                    a_mul: a_line.0,
                    b_mul: b_line.0,
                    total: prize_line.0,
                },
                LinearEquation {
                    a_mul: a_line.1,
                    b_mul: b_line.1,
                    total: prize_line.1,
                },
            )
        })
        .collect()
}

fn solve_system(system: &System) -> Option<(isize, isize)> {
    let (first, second) = system;
    let a_numerator = (first.total * second.b_mul) - (second.total * first.b_mul);
    let a_denominator = (first.a_mul * second.b_mul) - (first.b_mul * second.a_mul);
    if a_numerator % a_denominator != 0 {
        return None;
    }

    let a = a_numerator / a_denominator;

    let b_numerator = second.total - (second.a_mul * a);
    if b_numerator % second.b_mul != 0 {
        return None;
    }

    let b = b_numerator / second.b_mul;
    Some((a, b))
}

fn total_cost(systems: &Vec<System>, huge_prize: bool) -> usize {
    systems
        .iter()
        .map(|(first, second)| {
            (
                LinearEquation {
                    total: first.total + if huge_prize { 10000000000000 } else { 0 },
                    ..*first
                },
                LinearEquation {
                    total: second.total + if huge_prize { 10000000000000 } else { 0 },
                    ..*second
                },
            )
        })
        .filter_map(|s| solve_system(&s))
        .map(|(a, b)| ((a * 3) + b) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(total_cost(&parse_input(include_str!("../input-test.txt")), false) == 480)
    }
}
