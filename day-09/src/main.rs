use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free(u8),
    Taken(usize, u8),
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", move_blocks(&input));
    println!("Part 1: {}", move_files(&input));
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn checksum(files: &Vec<usize>) -> usize {
    files.iter().enumerate().map(|(i, id)| i * id).sum()
}

fn move_blocks(input: &Vec<u8>) -> usize {
    let free_spaces = input.into_iter().skip(1).step_by(2).collect::<Vec<_>>();
    let file_sizes = input.into_iter().step_by(2).collect::<Vec<_>>();
    let input_values = file_sizes
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| vec![i; (*v).into()])
        .collect::<Vec<_>>();

    let reverse_input = input_values.clone().into_iter().rev().collect::<Vec<_>>();

    let mut result = vec![];

    let mut i = 0;
    let mut front_read_values = 0usize;
    let mut end_read_values = 0usize;
    let total_size = input_values.len();

    loop {
        if i == file_sizes.len() || i == free_spaces.len() {
            break;
        }

        let Some(file_size) = min(
            Some((*file_sizes[i]) as usize),
            total_size
                .checked_sub(front_read_values)
                .and_then(|s| s.checked_sub(end_read_values)),
        ) else {
            break;
        };

        result.extend_from_slice(&input_values[front_read_values..(front_read_values + file_size)]);

        front_read_values += file_size;
        let Some(free_size) = min(
            Some((*free_spaces[i]) as usize),
            total_size
                .checked_sub(front_read_values)
                .and_then(|s| s.checked_sub(end_read_values)),
        ) else {
            break;
        };

        result.extend_from_slice(&reverse_input[end_read_values..(end_read_values + free_size)]);
        end_read_values += free_size;
        i += 1;
    }

    checksum(&result)
}

fn move_files(input: &Vec<u8>) -> usize {
    let blocks: Vec<_> = input
        .into_iter()
        .enumerate()
        .map(|(i, size)| {
            if i % 2 == 0 {
                Block::Taken(i / 2, *size)
            } else {
                Block::Free(*size)
            }
        })
        .filter(|block| match block {
            Block::Free(size) => *size > 0,
            _ => true,
        })
        .collect();

    let mut result = blocks.clone();

    for block in blocks.into_iter().rev() {
        let size = match block {
            Block::Taken(_, size) => size,
            _ => continue,
        };

        let free_index = result.iter().position(|block| match block {
            Block::Free(s) => *s >= size,
            _ => false,
        });

        if free_index.is_none() {
            continue;
        }

        let index = free_index.unwrap();

        let old_block_pos = result.iter().position(|b| *b == block).unwrap();

        if old_block_pos < index {
            continue;
        }

        let old_block = result.remove(old_block_pos);
        result.insert(old_block_pos, Block::Free(size));

        let old_free = result.remove(index);
        let space = match old_free {
            Block::Free(size) => size,
            _ => continue,
        };

        let diff = space - size;

        if diff > 0 {
            result.insert(index, Block::Free(diff));
        }

        result.insert(index, old_block);

        let mut new_result = vec![];
        let mut free_size = 0;
        for block in result {
            match block {
                Block::Free(s) => free_size += s,
                _ => {
                    if free_size > 0 {
                        new_result.push(Block::Free(free_size));
                        free_size = 0;
                    }
                    new_result.push(block);
                }
            }
        }

        result = new_result
    }

    checksum(
        &result
            .iter()
            .flat_map(|block| match block {
                Block::Free(size) => [0 as usize].repeat((*size) as usize),
                Block::Taken(id, size) => [*id].repeat((*size) as usize),
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(move_blocks(&parse_input(include_str!("../input-test.txt"))) == 1928);
    }

    #[test]
    fn example_2() {
        assert!(move_files(&parse_input(include_str!("../input-test.txt"))) == 2858);
    }
}
