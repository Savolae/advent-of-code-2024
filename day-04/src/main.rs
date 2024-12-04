fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect()
}

fn count_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> usize {
    if grid[y as usize][x as usize] != 'X' {
        return 0;
    }

    let is_xmas = |y_dir: i32, x_dir: i32| {
        (y + (y_dir * 3)) < grid.len() as i32
            && (y + (y_dir * 3)) >= 0
            && (x + (x_dir * 3)) < grid[0].len() as i32
            && (x + (x_dir * 3)) >= 0
            && (1..4)
                .map(|i| grid[((y_dir * i) + y) as usize][((x_dir * i) + x) as usize])
                .collect::<Vec<_>>()
                == ['M', 'A', 'S']
    };

    (-1..=1)
        .flat_map(|first| (-1..=1).map(move |second| (first, second)))
        .filter(|(y_dir, x_dir)| is_xmas(*y_dir, *x_dir))
        .count()
}

fn is_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let corners = [
        grid[y - 1][x - 1],
        grid[y + 1][x + 1],
        grid[y - 1][x + 1],
        grid[y + 1][x - 1],
    ];

    grid[y][x] == 'A'
        && corners.iter().all(|c| ['M', 'S'].contains(c))
        && corners[0] != corners[1]
        && corners[2] != corners[3]
}

fn part_1(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, _)| count_xmas(grid, x as i32, y as i32))
        })
        .sum()
}

fn part_2(grid: &Vec<Vec<char>>) -> usize {
    (1..grid.len() - 1)
        .flat_map(|y| (1..grid[0].len() - 1).map(move |x| is_x_mas(grid, x, y)))
        .filter(|xmas| *xmas)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = part_1(&parse_input(include_str!("../input-test.txt")));
        assert!(result == 18);
    }

    #[test]
    fn test_example_2() {
        let result = part_2(&parse_input(include_str!("../input-test.txt")));
        assert!(result == 9);
    }
}
